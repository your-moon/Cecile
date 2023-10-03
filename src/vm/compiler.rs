use std::{fmt::Display, hash::BuildHasherDefault, mem, ops::Range};

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};

use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::ast::{
        ExprAssign, ExprCall, ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, OpInfix,
        OpPrefix, Program, Span, Statement, StatementBlock, StatementExpr, StatementFor,
        StatementFun, StatementIf, StatementPrint, StatementPrintLn, StatementReturn, StatementVar,
        StatementWhile, Type,
    },
};

use super::{
    chunk::Chunk,
    object::{ObjectFunction, StringObject},
    op,
    value::Value,
};
use rustc_hash::FxHasher;

pub enum FunctionType {
    Script,
    Function,
}

#[derive(Debug, Default)]
struct Local {
    name: String,
    type_: Type,
    depth: usize,
    is_initialized: bool,
}

impl Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ name: {}, type_: {:?}, depth: {}, is_initialized: {} }}",
            self.name, self.type_, self.depth, self.is_initialized
        )
    }
}

pub struct CompilerCell {
    pub function: *mut ObjectFunction,
    type_: FunctionType,
    pub globals: HashMap<String, Type, BuildHasherDefault<FxHasher>>,
    locals: Vec<Local>,
    scope_depth: usize,
    parent: Option<Box<CompilerCell>>,
}

impl CompilerCell {
    pub fn resolve_local(&mut self, name: &str, span: &Span) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                if !local.is_initialized {
                    todo!("Can't read local variable in its own initializer");
                }
                return Some(i as u8);
            }
        }
        None
    }
}

pub struct Compiler {
    pub current_compiler: CompilerCell,
}

impl Compiler {
    pub fn new(allocator: &mut CeAllocation) -> Self {
        let name = allocator.alloc("");
        Self {
            current_compiler: CompilerCell {
                function: allocator.alloc(ObjectFunction::new(name, 0, None)),
                type_: FunctionType::Script,
                globals: HashMap::default(),
                locals: Vec::new(),
                scope_depth: 0,
                parent: None,
            },
        }
    }
    pub fn compile(
        &mut self,
        source: &mut Program,
        allocator: &mut CeAllocation,
    ) -> *mut ObjectFunction {
        for statement in &source.statements {
            self.compile_statement(statement, allocator);
        }

        self.emit_u8(op::NIL, &(0..0));
        self.emit_u8(op::RETURN, &(0..0));
        self.current_compiler.function
    }

    fn compile_statement(
        &mut self,
        (statement, range): &(Statement, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        match statement {
            Statement::Expression(expr) => self.compile_expression(&expr.expr, allocator),
            Statement::Print(print) => self.print_statement((print, range), allocator),
            Statement::PrintLn(print) => self.print_ln_statement((print, range), allocator),
            Statement::Var(var) => self.compile_statement_var((var, range), allocator),
            Statement::Block(block) => self.compile_statement_block((block, range), allocator),
            Statement::If(if_) => self.compile_statement_if((if_, range), allocator),
            Statement::While(while_) => self.compile_statement_while((while_, range), allocator),
            Statement::For(for_) => self.compile_statement_for((for_, range), allocator),
            Statement::Fun(func_) => {
                self.compile_statement_fun((func_, range), FunctionType::Function, allocator)
            }
            Statement::Return(return_) => {
                self.compile_statement_return((return_, range), allocator)
            }
            _ => todo!("statement not implemented"),
        }
    }

    fn compile_statement_return(
        &mut self,
        (return_, range): (&StatementReturn, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        match self.current_compiler.type_ {
            FunctionType::Script => {
                todo!("Can't return from top-level code");
            }
            FunctionType::Function => match &return_.value {
                Some(value) => {
                    let type_ = self.compile_expression(value, allocator);
                    let func_type = unsafe { &(*self.current_compiler.function).return_type };
                    println!("type: {:?} func_return_type:{:?}", type_, func_type);
                    match func_type {
                        Some(t) => {
                            if &type_ != t {
                                todo!("return type mismatch");
                            }
                            self.emit_u8(op::RETURN, &range);
                            return type_;
                        }
                        None => {
                            if type_ != Type::Nil {
                                todo!("return type must be nil")
                            }
                            self.emit_u8(op::RETURN, &range);
                            return type_;
                        }
                    }
                }
                None => {
                    self.emit_u8(op::NIL, &range);
                    self.emit_u8(op::RETURN, &range);
                    return Type::UnInitialized;
                }
            },
        }
    }

    fn compile_statement_fun(
        &mut self,
        (func, range): (&StatementFun, &Range<usize>),
        type_: FunctionType,
        allocator: &mut CeAllocation,
    ) -> Type {
        let name = allocator.alloc(&func.name);
        let arity = func.params.len() as u8;
        let cell = CompilerCell {
            function: allocator.alloc(ObjectFunction::new(name, arity, func.return_type.clone())),
            type_: FunctionType::Function,
            globals: HashMap::default(),
            locals: Vec::new(),
            scope_depth: self.current_compiler.scope_depth + 1,
            parent: None,
        };
        self.begin_cell(cell);

        match type_ {
            FunctionType::Script => {
                self.current_compiler
                    .globals
                    .insert(func.name.clone(), Type::Object);
            }
            FunctionType::Function => {
                self.declare_local(&func.name, &Type::Object, &range);
            }
        }
        //TODO check param type
        for (param_string, param_type) in &func.params {
            match param_type {
                Some(t) => match t {
                    Type::String | Type::Int => {
                        self.declare_local(&param_string, t, &range);
                    }
                    _ => todo!("type not implemented"),
                },
                None => {
                    self.declare_local(&param_string, &Type::UnInitialized, &range);
                }
            }
            self.define_local();
        }

        for statement in &func.body.statements {
            self.compile_statement(&statement, allocator);
        }

        if unsafe { (*self.current_compiler.function).chunk.code.last().unwrap() } != &op::RETURN {
            self.emit_u8(op::NIL, &range);
            self.emit_u8(op::RETURN, &range);
        }

        let function = self.end_cell();
        self.emit_u8(op::DEFINE_GLOBAL, &range);
        self.write_constant(Value::Function(function), &range);

        // println!("{:?}", unsafe { (*function).chunk.disassemble(&func.name) });

        return Type::UnInitialized;
    }

    fn begin_cell(&mut self, cell: CompilerCell) {
        let cell = mem::replace(&mut self.current_compiler, cell);
        self.current_compiler.parent = Some(Box::new(cell));
    }

    fn end_cell(&mut self) -> *mut ObjectFunction {
        let parent = self
            .current_compiler
            .parent
            .take()
            .expect("Compiler has no parent");
        let cell = mem::replace(&mut self.current_compiler, *parent);
        cell.function
    }

    fn compile_statement_for(
        &mut self,
        (for_, range): (&StatementFor, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        self.begin_scope();
        let mut compiled_type = None;
        if let Some(init) = &for_.init {
            compiled_type = Some(self.compile_statement(&init, allocator));
        }
        let loop_start = unsafe { (*self.current_compiler.function).chunk.code.len() };
        let mut exit_jump = None;
        if let Some(cond) = &for_.cond {
            let cond_type = self.compile_expression(cond, allocator);
            if cond_type != Type::Bool {
                todo!("for condition must be bool");
            }
            exit_jump = Some(self.emit_jump(op::JUMP_IF_FALSE, &range));
            self.emit_u8(op::POP, &range);
        }

        self.compile_statement(&for_.body, allocator);
        if let Some(increment) = &for_.update {
            self.compile_expression(increment, allocator);
            // self.emit_u8(op::POP, &range);
        }
        self.emit_loop(loop_start, &range);

        if let Some(exit_jump) = exit_jump {
            self.patch_jump(exit_jump, &range);
            self.emit_u8(op::POP, &range);
        }
        self.end_scope(range.clone(), allocator);
        if let Some(compiled_type) = compiled_type {
            return compiled_type;
        }
        return Type::UnInitialized;
    }

    fn compile_statement_while(
        &mut self,
        (while_, range): (&StatementWhile, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let loop_start = unsafe { (*self.current_compiler.function).chunk.code.len() };
        let cond_type = self.compile_expression(&while_.cond, allocator);
        if cond_type != Type::Bool {
            todo!("while condition must be bool");
        }
        let exit_jump = self.emit_jump(op::JUMP_IF_FALSE, &range);
        self.emit_u8(op::POP, &range);
        self.compile_statement(&while_.body, allocator);
        self.emit_loop(loop_start, &range);
        self.patch_jump(exit_jump, &range);
        self.emit_u8(op::POP, &range);
        return Type::UnInitialized;
    }

    fn emit_loop(&mut self, loop_start: usize, span: &Span) {
        self.emit_u8(op::LOOP, span);
        let offset = unsafe { (*self.current_compiler.function).chunk.code.len() - loop_start + 2 };
        if offset > u16::MAX as usize {
            todo!("Loop body too large");
        }
        self.emit_u8((offset >> 8) as u8, span);
        self.emit_u8(offset as u8, span);
    }

    fn compile_statement_if(
        &mut self,
        (if_, range): (&StatementIf, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let cond_type = self.compile_expression(&if_.cond, allocator);
        if cond_type != Type::Bool {
            todo!("if condition must be bool");
        }
        // If the condition is false, go to ELSE.
        let jump_to_else = self.emit_jump(op::JUMP_IF_FALSE, &range);
        // Discard the condition.
        self.emit_u8(op::POP, &range);
        // Evaluate the if branch.
        self.compile_statement(&if_.then_branch, allocator);
        // Go to END.
        let jump_to_end = self.emit_jump(op::JUMP, &range);

        // ELSE:
        self.patch_jump(jump_to_else, &range);
        self.emit_u8(op::POP, &range); // Discard the condition.
        if let Some(else_branch) = &if_.else_branch {
            self.compile_statement(else_branch, allocator);
        }

        // END:
        self.patch_jump(jump_to_end, &range);
        return Type::UnInitialized;
    }

    fn compile_statement_block(
        &mut self,
        (block, range): (&StatementBlock, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        self.begin_scope();
        for statement in &block.statements {
            self.compile_statement(&statement, allocator);
        }
        self.end_scope(range.clone(), allocator);
        return Type::UnInitialized;
    }

    fn print_ln_statement(
        &mut self,
        print: (&StatementPrintLn, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (print, range) = print;
        let expr_type = self.compile_expression(&print.value, allocator);
        self.emit_u8(op::PRINT_LN, range);
        return expr_type;
    }

    fn print_statement(
        &mut self,
        print: (&StatementPrint, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (print, range) = print;
        let expr_type = self.compile_expression(&print.value, allocator);
        self.emit_u8(op::PRINT, range);
        return expr_type;
    }

    fn compile_expression(
        &mut self,
        expr: &(Expression, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (expr, range) = expr;
        match expr {
            Expression::Assign(assign) => self.compile_assign((assign, range), allocator),
            Expression::Prefix(prefix) => self.compile_prefix((prefix, range), allocator),
            Expression::Infix(infix) => self.compile_infix((infix, range), allocator),
            Expression::Literal(value) => self.compile_literal((value, range), allocator),
            Expression::Var(var) => self.compile_expression_var((var, range), allocator),
            Expression::Call(call) => self.compile_call((call, range), allocator),
            _ => todo!("expression not implemented"),
        }
    }

    fn compile_call(
        &mut self,
        (call, range): (&Box<ExprCall>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let arg_count = call.args.len();
        if arg_count > u8::MAX as usize {
            todo!("Too many arguments, Can't call more than 255 arguments");
        }
        let callee_type = self.compile_expression(&call.callee, allocator);
        // if callee_type != Type::Object {
        //     todo!("Can only call functions and classes");
        // }
        for arg in &call.args {
            self.compile_expression(&arg, allocator);
        }

        self.emit_u8(op::CALL, &range);
        self.emit_u8(arg_count as u8, &range);
        callee_type
    }

    fn compile_assign(
        &mut self,
        (assign, range): (&Box<ExprAssign>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let var_type = self.compile_expression(&assign.rhs, allocator);
        if let Some(local_index) = self
            .current_compiler
            .resolve_local(&assign.lhs.name, &range)
        {
            self.emit_u8(op::SET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
        } else {
            let name = allocator.alloc(&assign.lhs.name);
            self.emit_u8(op::SET_GLOBAL, &range);
            self.write_constant(Value::String(name), &range);
        }
        return var_type;
    }

    fn compile_statement_var(
        &mut self,
        (var, range): (&StatementVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let var_type = var.value.as_ref().map_or(Type::UnInitialized, |value| {
            self.compile_expression(value, allocator)
        });

        match &var.var.type_ {
            Some(t) => match t {
                Type::String | Type::Int => {
                    let name = &var.var.name;
                    if self.is_global() {
                        let string = allocator.alloc(name);
                        self.current_compiler
                            .globals
                            .insert(name.clone(), t.clone());
                        self.emit_u8(op::DEFINE_GLOBAL, &range);
                        self.write_constant(Value::String(string), &range);
                        return Type::String;
                    } else {
                        self.declare_local(name, &var_type, &range);
                    }
                }
                _ => todo!("type not implemented"),
            },
            None => {
                let name = &var.var.name;
                let string = allocator.alloc(name);
                if self.is_global() {
                    self.current_compiler
                        .globals
                        .insert(name.clone(), var_type.clone());
                    self.emit_u8(op::DEFINE_GLOBAL, &range);
                    self.write_constant(Value::String(string), &range);
                } else {
                    self.declare_local(name, &var_type, &range);
                }
            }
        }
        return var_type;
    }

    fn compile_expression_var(
        &mut self,
        (prefix, range): (&ExprVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let name = &prefix.var.name;
        if let Some(local_index) = self
            .current_compiler
            .resolve_local(&prefix.var.name, &range)
        {
            self.emit_u8(op::GET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
            self.current_compiler.locals[local_index as usize]
                .type_
                .clone()
        } else {
            let string = allocator.alloc(name);
            self.emit_u8(op::GET_GLOBAL, &range);
            self.write_constant(Value::String(string), &range);
            let type_ = self.current_compiler.globals.get(&*name);
            return match type_ {
                Some(t) => return t.clone(),
                None => Type::UnInitialized,
            };
        }
    }

    fn compile_prefix(
        &mut self,
        prefix: (&Box<ExprPrefix>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (prefix, range) = prefix;
        let rt_type = self.compile_expression(&(prefix.rt), allocator);
        if rt_type != Type::Int || rt_type != Type::Bool {
            todo!("prefix type mismatch");
        }
        match prefix.op {
            OpPrefix::Negate => self.emit_u8(op::NEG, &range),
            OpPrefix::Not => self.emit_u8(op::NOT, &range),
        }
        return rt_type;
    }

    fn compile_infix(
        &mut self,
        infix: (&Box<ExprInfix>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (infix, range) = infix;
        let lhs_type = self.compile_expression(&(infix.lhs), allocator);
        let rhs_type = self.compile_expression(&(infix.rhs), allocator);
        println!("lhs: {:?} rhs: {:?}", lhs_type, rhs_type);
        //if lhs_type: String rhs_type: String => concat string command will be called
        //if lhs_type: Int  rhs_type: Int  => add int command will be called
        // let return_type = match (lhs_type, rhs_type) {
        //     (Type::String, Type::String) => Type::String,
        //     (Type::String, Type::Int) => Type::String,
        //     (Type::Int, Type::String) => Type::String,
        //     (Type::Int, Type::Int) => Type::Int,
        //     _ => todo!("type mismatch"),
        // };
        let return_type = lhs_type;

        // if lhs_type != rhs_type {
        //     todo!("type mismatch");
        // }

        match infix.op {
            OpInfix::Modulo => {
                self.emit_u8(op::MODULO, &range);
                return return_type;
            }
            OpInfix::Add => {
                self.emit_u8(op::ADD, &range);
                return return_type;
            }
            OpInfix::Sub => {
                return return_type;
            }
            OpInfix::Mul => {
                self.emit_u8(op::MUL, &range);
                return return_type;
            }
            OpInfix::Div => {
                self.emit_u8(op::DIV, &range);
                return return_type;
            }
            OpInfix::Equal => {
                self.emit_u8(op::EQUAL, &range);
                return Type::Bool;
            }
            OpInfix::NotEqual => {
                self.emit_u8(op::NOT_EQUAL, &range);
                return Type::Bool;
            }
            OpInfix::Greater => {
                self.emit_u8(op::GREATER_THAN, &range);
                return Type::Bool;
            }
            OpInfix::GreaterEqual => {
                self.emit_u8(op::GREATER_THAN_EQUAL, &range);
                return Type::Bool;
            }
            OpInfix::Less => {
                self.emit_u8(op::LESS_THAN, &range);
                return Type::Bool;
            }
            OpInfix::LessEqual => {
                self.emit_u8(op::LESS_THAN_EQUAL, &range);
                return Type::Bool;
            }
            OpInfix::LogicOr => {
                self.emit_u8(op::OR, &range);
                return Type::Bool;
            }
            OpInfix::LogicAnd => {
                self.emit_u8(op::AND, &range);
                return Type::Bool;
            }
        }
    }

    fn compile_literal(
        &mut self,
        (literal, range): (&ExprLiteral, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        match literal {
            ExprLiteral::Number(value) => {
                self.emit_constant(Value::Number(*value), &range);
                Type::Int
            }
            ExprLiteral::String(string) => {
                let string = allocator.alloc(string);
                self.emit_constant(Value::String(string), &range);
                Type::String
            }
            ExprLiteral::Bool(value) => {
                match value {
                    true => self.emit_u8(op::TRUE, &range),
                    false => self.emit_u8(op::FALSE, &range),
                };
                Type::Bool
            }
            ExprLiteral::Nil => {
                self.emit_u8(op::NIL, &range);
                Type::Nil
            }
        }
    }

    fn begin_scope(&mut self) {
        self.current_compiler.scope_depth += 1;
    }

    fn end_scope(&mut self, span: Range<usize>, allocator: &mut CeAllocation) {
        self.current_compiler.scope_depth -= 1;
        while !self.current_compiler.locals.is_empty()
            && self.current_compiler.locals.last().unwrap().depth
                > self.current_compiler.scope_depth
        {
            let local = self.current_compiler.locals.pop().unwrap();
            self.emit_u8(op::POP, &span);
        }
    }

    fn declare_local(&mut self, name: &str, type_: &Type, span: &Span) {
        let is_initialized = match type_ {
            Type::UnInitialized => false,
            _ => true,
        };
        for local in self.current_compiler.locals.iter().rev() {
            if local.depth != self.current_compiler.scope_depth {
                break;
            }

            if local.name == name {
                todo!("Already declared variable in this scope");
            }
        }

        let local = Local {
            name: name.to_string(),
            type_: type_.clone(),
            depth: self.current_compiler.scope_depth,
            is_initialized,
        };
        self.current_compiler.locals.push(local);
    }

    fn define_local(&mut self) {
        let local = self
            .current_compiler
            .locals
            .last_mut()
            .expect("Define local called without locals");
        local.is_initialized = true;
    }

    fn is_global(&self) -> bool {
        self.current_compiler.scope_depth == 0
    }

    fn emit_jump(&mut self, instruction: u8, span: &Span) -> usize {
        self.emit_u8(instruction, span);
        self.emit_u8(0xff, span);
        self.emit_u8(0xff, span);
        unsafe { (*self.current_compiler.function).chunk.code.len() - 2 }
    }

    fn patch_jump(&mut self, offset_idx: usize, span: &Span) {
        let offset = unsafe { (*self.current_compiler.function).chunk.code.len() - offset_idx - 2 };
        if offset > u16::MAX as usize {
            todo!("Too much code to jump over");
        }
        unsafe {
            (*self.current_compiler.function).chunk.code[offset_idx] = (offset >> 8) as u8;
            (*self.current_compiler.function).chunk.code[offset_idx + 1] = offset as u8;
        }
    }

    fn emit_u8(&mut self, byte: u8, span: &Span) {
        unsafe {
            (*self.current_compiler.function).chunk.emit_u8(byte, span);
        }
    }

    fn emit_constant(&mut self, value: Value, span: &Span) {
        unsafe {
            (*self.current_compiler.function)
                .chunk
                .emit_constant(value, span);
        }
    }

    fn write_constant(&mut self, value: Value, span: &Span) {
        unsafe {
            (*self.current_compiler.function)
                .chunk
                .write_constant(value, span);
        }
    }
}
