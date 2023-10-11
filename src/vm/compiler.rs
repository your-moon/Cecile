use std::{fmt::Display, hash::BuildHasherDefault, mem, ops::Range};

use arrayvec::ArrayVec;
use hashbrown::{hash_map::DefaultHashBuilder, HashMap};

use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::{
        ast::{
            ExprAssign, ExprCall, ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, OpInfix,
            OpPrefix, Program, Span, Statement, StatementBlock, StatementExpr, StatementFor,
            StatementFun, StatementIf, StatementPrint, StatementPrintLn, StatementReturn,
            StatementVar, StatementWhile, Type,
        },
        parse,
    },
    vm::error::TypeError,
};

use super::{
    chunk::Chunk,
    error::{Error, ErrorS, Result, SyntaxError},
    object::{ObjectFunction, StringObject},
    op,
    value::Value,
};
use rustc_hash::FxHasher;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Upvalue {
    index: u8,
    is_local: bool,
}

pub struct CompilerCell {
    pub function: *mut ObjectFunction,
    type_: FunctionType,
    pub globals: HashMap<String, Type, BuildHasherDefault<FxHasher>>,
    locals: ArrayVec<Local, 256>,
    scope_depth: usize,
    parent: Option<Box<CompilerCell>>,
    upvalues: Vec<Upvalue>,
}

impl CompilerCell {
    pub fn resolve_local(&mut self, name: &str, span: &Span) -> Option<u8> {
        match self
            .locals
            .iter_mut()
            .enumerate()
            .rfind(|(_, local)| local.name == name)
        {
            Some((idx, local)) => {
                if local.is_initialized {
                    Some(idx.try_into().expect("local index overflow"))
                } else {
                    todo!("local used before initialization")
                }
            }
            None => None,
        }
    }

    pub fn resolve_upvalue(&mut self, name: &str, span: &Span) -> Option<u8> {
        if let Some(parent) = &mut self.parent {
            if let Some(local) = parent.resolve_local(name, span) {
                return Some(self.add_upvalue(local, true));
            }
            if let Some(upvalue) = parent.resolve_upvalue(name, span) {
                return Some(self.add_upvalue(upvalue, false));
            }
        }
        None
    }

    pub fn add_upvalue(&mut self, index: u8, is_local: bool) -> u8 {
        let upvalue = Upvalue { index, is_local };
        let upvalue_idx = match self.upvalues.iter().position(|u| u == &upvalue) {
            Some(idx) => idx,
            None => {
                self.upvalues.push(upvalue);
                let upvalue_cnt = self.upvalues.len();
                unsafe { (*self.function).upvalue_count = upvalue_cnt as u16 };
                upvalue_cnt - 1
            }
        };
        upvalue_idx as u8
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
                locals: ArrayVec::new(),
                scope_depth: 0,
                parent: None,
                upvalues: Vec::new(),
            },
        }
    }
    pub fn compile(
        &mut self,
        source: &str,
        allocator: &mut CeAllocation,
    ) -> Result<*mut ObjectFunction, Vec<ErrorS>> {
        let program = parse(source)?;

        for statement in &program.statements {
            let (type_, result) = self.compile_statement(statement, allocator);
            if let Err(e) = result {
                return Err(vec![e]);
            }
        }

        self.emit_u8(op::NIL, &(0..0));
        self.emit_u8(op::RETURN, &(0..0));
        unsafe {
            (*self.current_compiler.function)
                .chunk
                .disassemble("script")
        };
        Ok(self.current_compiler.function)
    }

    fn compile_statement(
        &mut self,
        (statement, range): &(Statement, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        match statement {
            Statement::Expression(expr) => {
                let (type_, result) = self.compile_expression(&expr.expr, allocator);
                if let Err(e) = result {
                    return (type_, Err(e));
                }
                self.emit_u8(op::POP, &range);
                return (type_, Ok(()));
            }
            Statement::Print(print) => self.print_statement((print, range), allocator),
            Statement::PrintLn(print) => self.print_ln_statement((print, range), allocator),
            Statement::Var(var) => self.compile_statement_var((var, range), allocator),
            Statement::Block(block) => self.compile_statement_block((block, range), allocator),
            Statement::If(if_) => self.compile_statement_if((if_, range), allocator),
            Statement::While(while_) => self.compile_statement_while((while_, range), allocator),
            Statement::For(for_) => self.compile_statement_for((for_, range), allocator),
            Statement::Fun(func) => {
                let (func_type, result) =
                    self.compile_statement_fun((func, range), FunctionType::Function, allocator);
                if let Err(e) = result {
                    return (func_type, Err(e));
                }
                // println!("compiled func return type: {:?}", func_type);
                if self.is_global() {
                    let name = allocator.alloc(&func.name);
                    self.current_compiler
                        .globals
                        .insert(func.name.clone(), Type::Fn);
                    self.emit_u8(op::DEFINE_GLOBAL, &range);
                    self.write_constant(Value::String(name), &range);
                } else {
                    self.declare_local(&func.name, &Type::Fn, &range);
                }
                return (func_type, Ok(()));
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
    ) -> (Type, Result<()>) {
        match self.current_compiler.type_ {
            FunctionType::Script => {
                let result = Err((
                    Error::SyntaxError(SyntaxError::ReturnInInitializer),
                    range.clone(),
                ));
                return (Type::Nil, result);
            }
            FunctionType::Function => {
                let func_type_ = unsafe { &(*self.current_compiler.function).return_type };
                match &return_.value {
                    Some(value) => {
                        let (return_type, result) = self.compile_expression(value, allocator);
                        if let Err(e) = result {
                            return (return_type, Err(e));
                        }
                        println!(
                            "return type: {:?} func_return_type:{:?}",
                            return_type, func_type_
                        );
                        match func_type_ {
                            Some(t) => {
                                if &return_type != t {
                                    let result = Err((
                                        Error::TypeError(TypeError::ReturnTypeMismatch {
                                            expected: t.to_string(),
                                            actual: return_type.to_string(),
                                        }),
                                        range.clone(),
                                    ));
                                    return (return_type, result);
                                }
                                self.emit_u8(op::RETURN, &range);
                                return (return_type, Ok(()));
                            }
                            None => {
                                if return_type != Type::Nil {
                                    let result = Err((
                                        Error::TypeError(TypeError::ReturnTypeMustBeNil),
                                        range.clone(),
                                    ));
                                    return (return_type, result);
                                }
                                self.emit_u8(op::RETURN, &range);
                                return (return_type, Ok(()));
                            }
                        }
                    }
                    None => {
                        match func_type_ {
                            Some(t) => {
                                let result = Err((
                                    Error::TypeError(TypeError::ReturnTypeMismatch {
                                        expected: t.to_string(),
                                        actual: Type::Nil.to_string(),
                                    }),
                                    range.clone(),
                                ));
                                return (Type::Nil, result);
                            }
                            _ => {
                                self.emit_u8(op::NIL, &range);
                                self.emit_u8(op::RETURN, &range);
                            }
                        }
                        return (Type::UnInitialized, Ok(()));
                    }
                }
            }
        }
    }

    fn compile_statement_fun(
        &mut self,
        (func, range): (&StatementFun, &Range<usize>),
        type_: FunctionType,
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let name = allocator.alloc(&func.name);
        let arity_count = func.params.len() as u8;
        // let mut arity = Vec::new();
        // for (param_string, param_type) in &func.params {
        //     let param = Arity {
        //         name: allocator.alloc(param_string),
        //         type_: param_type.clone().unwrap(),
        //     };
        //     arity.push(param);
        // }
        let cell = CompilerCell {
            function: allocator.alloc(ObjectFunction::new(
                name,
                arity_count,
                func.return_type.clone(),
            )),
            type_: FunctionType::Function,
            globals: HashMap::default(),
            locals: ArrayVec::new(),
            scope_depth: self.current_compiler.scope_depth + 1,
            parent: None,
            upvalues: Vec::new(),
        };
        self.begin_cell(cell);

        match type_ {
            FunctionType::Script => {
                self.current_compiler
                    .globals
                    .insert(func.name.clone(), Type::Fn);
            }
            FunctionType::Function => {
                self.declare_local(&func.name, &Type::Fn, &range);
            }
        }
        //TODO check param type
        let mut param_strings = Vec::new();
        for (param_string, param_type) in &func.params {
            param_strings.push((param_string, param_type));
        }
        for (param_string, param_type) in param_strings.iter().rev() {
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
            let stmt = (
                Statement::Return(StatementReturn { value: None }),
                range.clone(),
            );
            let (stmt_type, result) = self.compile_statement(&stmt, allocator);
            if let Err(e) = result {
                return (stmt_type, Err(e));
            }
        }

        let (function, upvalues) = self.end_cell();
        unsafe { (*function).chunk.disassemble((*name).value) };
        self.emit_u8(op::CLOSURE, &range);
        self.write_constant(Value::Function(function), &range);
        for upvalue in &upvalues {
            self.emit_u8(upvalue.is_local.into(), &range);
            self.emit_u8(upvalue.index, &range);
        }

        let return_type = match &func.return_type {
            Some(t) => t.clone(),
            None => Type::Nil,
        };
        return (return_type, Ok(()));
    }

    fn begin_cell(&mut self, cell: CompilerCell) {
        let cell = mem::replace(&mut self.current_compiler, cell);
        self.current_compiler.parent = Some(Box::new(cell));
    }

    fn end_cell(&mut self) -> (*mut ObjectFunction, Vec<Upvalue>) {
        let parent = self
            .current_compiler
            .parent
            .take()
            .expect("Compiler has no parent");
        let cell = mem::replace(&mut self.current_compiler, *parent);
        (cell.function, cell.upvalues)
    }

    fn compile_statement_for(
        &mut self,
        (for_, range): (&StatementFor, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        self.begin_scope();
        let mut compiled_type = None;
        if let Some(init) = &for_.init {
            let (compile_init, result) = self.compile_statement(&init, allocator);
            if let Err(e) = result {
                return (compile_init, Err(e));
            }
            compiled_type = Some(compile_init);
        }
        let loop_start = unsafe { (*self.current_compiler.function).chunk.code.len() };
        let mut exit_jump = None;
        if let Some(cond) = &for_.cond {
            let (cond_type, result) = self.compile_expression(cond, allocator);
            if let Err(e) = result {
                return (cond_type, Err(e));
            }
            if cond_type != Type::Bool {
                let result = Err((
                    Error::TypeError(TypeError::LoopMustBeBoolean),
                    range.clone(),
                ));
                return (cond_type, result);
            }
            exit_jump = Some(self.emit_jump(op::JUMP_IF_FALSE, &range));
            self.emit_u8(op::POP, &range);
        }

        let (body_type, result) = self.compile_statement(&for_.body, allocator);
        if let Err(e) = result {
            return (body_type, Err(e));
        }
        if let Some(increment) = &for_.update {
            let (incr_type, result) = self.compile_expression(increment, allocator);
            if let Err(e) = result {
                return (incr_type, Err(e));
            }
            self.emit_u8(op::POP, &range);
        }
        self.emit_loop(loop_start, &range);

        if let Some(exit_jump) = exit_jump {
            self.patch_jump(exit_jump, &range);
            self.emit_u8(op::POP, &range);
        }
        self.end_scope(range.clone());
        if let Some(compiled_type) = compiled_type {
            return (compiled_type, Ok(()));
        }
        return (Type::UnInitialized, Ok(()));
    }

    fn compile_statement_while(
        &mut self,
        (while_, range): (&StatementWhile, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let loop_start = unsafe { (*self.current_compiler.function).chunk.code.len() };
        let (cond_type, result) = self.compile_expression(&while_.cond, allocator);
        if let Err(e) = result {
            return (cond_type, Err(e));
        }
        if cond_type != Type::Bool {
            let result = Err((
                Error::TypeError(TypeError::LoopMustBeBoolean),
                range.clone(),
            ));
            return (cond_type, result);
        }
        let exit_jump = self.emit_jump(op::JUMP_IF_FALSE, &range);
        self.emit_u8(op::POP, &range);
        let (body_type, result) = self.compile_statement(&while_.body, allocator);
        if let Err(e) = result {
            return (body_type, Err(e));
        }
        self.emit_loop(loop_start, &range);
        self.patch_jump(exit_jump, &range);
        self.emit_u8(op::POP, &range);
        return (Type::UnInitialized, Ok(()));
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
    ) -> (Type, Result<()>) {
        let (cond_type, result) = self.compile_expression(&if_.cond, allocator);
        if let Err(e) = result {
            return (cond_type, Err(e));
        }
        if cond_type != Type::Bool {
            let result = Err((
                Error::TypeError(TypeError::LoopMustBeBoolean),
                range.clone(),
            ));
            return (cond_type, result);
        }
        // If the condition is false, go to ELSE.
        let jump_to_else = self.emit_jump(op::JUMP_IF_FALSE, &range);
        // Discard the condition.
        self.emit_u8(op::POP, &range);
        // Evaluate the if branch.
        let (then_type, result) = self.compile_statement(&if_.then_branch, allocator);
        if let Err(e) = result {
            return (then_type, Err(e));
        }
        // Go to END.
        let jump_to_end = self.emit_jump(op::JUMP, &range);

        // ELSE:
        self.patch_jump(jump_to_else, &range);
        self.emit_u8(op::POP, &range); // Discard the condition.
        if let Some(else_branch) = &if_.else_branch {
            let (else_type, result) = self.compile_statement(else_branch, allocator);
            if let Err(e) = result {
                return (else_type, Err(e));
            }
        }

        // END:
        self.patch_jump(jump_to_end, &range);
        return (Type::UnInitialized, Ok(()));
    }

    fn compile_statement_block(
        &mut self,
        (block, range): (&StatementBlock, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        self.begin_scope();
        for statement in &block.statements {
            let (type_, result) = self.compile_statement(&statement, allocator);
            if let Err(e) = result {
                return (type_, Err(e));
            }
        }
        self.end_scope(range.clone());
        return (Type::UnInitialized, Ok(()));
    }

    fn print_ln_statement(
        &mut self,
        print: (&StatementPrintLn, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let (print, range) = print;
        let (expr_type, result) = self.compile_expression(&print.value, allocator);
        if let Err(e) = result {
            return (expr_type, Err(e));
        }
        self.emit_u8(op::PRINT_LN, range);
        return (expr_type, Ok(()));
    }

    fn print_statement(
        &mut self,
        print: (&StatementPrint, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let (print, range) = print;
        let (expr_type, result) = self.compile_expression(&print.value, allocator);
        if let Err(e) = result {
            return (expr_type, Err(e));
        }
        self.emit_u8(op::PRINT, range);
        return (expr_type, Ok(()));
    }

    fn compile_expression(
        &mut self,
        expr: &(Expression, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
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
    ) -> (Type, Result<()>) {
        let arg_count = call.args.len();
        if arg_count > u8::MAX as usize {
            todo!("Too many arguments, Can't call more than 255 arguments");
        }
        let (callee_type, result) = self.compile_expression(&call.callee, allocator);
        if let Err(e) = result {
            return (callee_type, Err(e));
        }
        // if callee_type != Type::Object {
        //     todo!("Can only call functions and classes");
        // }
        for arg in &call.args {
            let (arg_type, result) = self.compile_expression(&arg, allocator);
            if let Err(e) = result {
                return (arg_type, Err(e));
            }
        }

        self.emit_u8(op::CALL, &range);
        self.emit_u8(arg_count as u8, &range);
        (callee_type, Ok(()))
    }

    fn compile_assign(
        &mut self,
        (assign, range): (&Box<ExprAssign>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let (var_type, result) = self.compile_expression(&assign.rhs, allocator);
        if let Err(e) = result {
            return (var_type, Err(e));
        }
        if let Some(local_index) = self
            .current_compiler
            .resolve_local(&assign.lhs.name, &range)
        {
            self.emit_u8(op::SET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
        } else if let Some(upvalue) = self
            .current_compiler
            .resolve_upvalue(&assign.lhs.name, &range)
        {
            self.emit_u8(op::SET_UPVALUE, &range);
            self.write_constant(Value::Number(upvalue.into()), &range);
        } else {
            let name = allocator.alloc(&assign.lhs.name);
            self.emit_u8(op::SET_GLOBAL, &range);
            self.write_constant(Value::String(name), &range);
        }
        return (var_type, Ok(()));
    }

    fn compile_statement_var(
        &mut self,
        (var, range): (&StatementVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        // Variable declaration left hand side expression, if it's not have expression variable
        // type is UnInitialized
        let (var_type, result) =
            var.value
                .as_ref()
                .map_or((Type::UnInitialized, Ok(())), |value| {
                    let (value_type, result) = self.compile_expression(value, allocator);
                    if let Err(e) = result {
                        return (value_type, Err(e));
                    }
                    return (value_type, Ok(()));
                });

        if let Err(e) = result {
            return (var_type, Err(e));
        }

        //Check if variable type is declared
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
                        return (Type::String, Ok(()));
                    } else {
                        self.declare_local(name, &var_type, &range);
                    }
                }
                _ => todo!("type not implemented"),
            },
            None => {
                let name = &var.var.name;
                let string = allocator.alloc(name);
                //If variable type is not declared, left hand side expression type will be variable type
                if self.is_global() {
                    self.current_compiler
                        .globals
                        .insert(name.clone(), var_type.clone());
                    self.emit_u8(op::DEFINE_GLOBAL, &range);
                    self.write_constant(Value::String(string), &range);
                } else {
                    //If variable type is not declared, left hand side expression type will be variable type
                    self.declare_local(name, &var_type, &range);
                }
            }
        }
        return (var_type, Ok(()));
    }

    fn compile_expression_var(
        &mut self,
        (prefix, range): (&ExprVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let name = &prefix.var.name;
        // if self.current_compiler.type_ == FunctionType::Function {
        //     let function_ = unsafe { &mut (*self.current_compiler.function) };
        //     let arity = function_.arity.as_ref().unwrap();
        //     for param in arity {
        //         if unsafe { (*param.name).value } == name {
        //             return param.type_.clone();
        //         }
        //     }
        // }
        if let Some(local_index) = self
            .current_compiler
            .resolve_local(&prefix.var.name, &range)
        {
            self.emit_u8(op::GET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
            let expr_var_type = self.current_compiler.locals[local_index as usize]
                .type_
                .clone();
            println!(
                "expr var type: {:?}, name: {:?}",
                expr_var_type, &prefix.var.name
            );
            return (expr_var_type, Ok(()));
        } else if let Some(upvalue) = self.current_compiler.resolve_upvalue(name, &range) {
            self.emit_u8(op::GET_UPVALUE, &range);
            self.write_constant(Value::Number(upvalue.into()), &range);
            return (Type::UnInitialized, Ok(()));
        } else {
            let string = allocator.alloc(name);
            self.emit_u8(op::GET_GLOBAL, &range);
            self.write_constant(Value::String(string), &range);
            let type_ = self.current_compiler.globals.get(&*name);
            let expr_var_type = match type_ {
                Some(t) => t.clone(),
                None => Type::UnInitialized,
            };
            println!(
                "expr var type: {:?}, name: {:?}",
                expr_var_type, &prefix.var.name
            );
            return (expr_var_type, Ok(()));
        }
    }

    fn compile_prefix(
        &mut self,
        prefix: (&Box<ExprPrefix>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let (prefix, range) = prefix;
        let (rt_type, expr) = self.compile_expression(&(prefix.rt), allocator);
        if let Err(e) = expr {
            return (rt_type, Err(e));
        }
        if rt_type != Type::Int || rt_type != Type::Bool {
            let spanned_error = Err((
                Error::TypeError(TypeError::UnsupportedOperandPrefix {
                    op: prefix.op.to_string(),
                    rt_type: rt_type.to_string(),
                }),
                range.clone(),
            ));
            return (rt_type, spanned_error);
        }
        match prefix.op {
            OpPrefix::Negate => self.emit_u8(op::NEG, &range),
            OpPrefix::Not => self.emit_u8(op::NOT, &range),
        }
        return (rt_type, Ok(()));
    }

    fn compile_infix(
        &mut self,
        infix: (&Box<ExprInfix>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        let (infix, range) = infix;
        let (lhs_type, result) = self.compile_expression(&(infix.lhs), allocator);
        if let Err(e) = result {
            return (lhs_type, Err(e));
        }
        let (rhs_type, result) = self.compile_expression(&(infix.rhs), allocator);
        if let Err(e) = result {
            return (rhs_type, Err(e));
        }

        println!("lhs: {:?} rhs: {:?}", lhs_type, rhs_type);
        //if lhs_type: String rhs_type: String => concat string command will be called
        //if lhs_type: Int  rhs_type: Int  => add int command will be called
        let return_type = lhs_type;

        // if lhs_type != rhs_type {
        //     todo!("type mismatch");
        // }

        match infix.op {
            OpInfix::Modulo => {
                self.emit_u8(op::MODULO, &range);
                return (return_type, Ok(()));
            }
            OpInfix::Add => {
                self.emit_u8(op::ADD, &range);
                return (return_type, Ok(()));
            }
            OpInfix::Sub => {
                self.emit_u8(op::SUB, &range);
                return (return_type, Ok(()));
            }
            OpInfix::Mul => {
                self.emit_u8(op::MUL, &range);
                return (return_type, Ok(()));
            }
            OpInfix::Div => {
                self.emit_u8(op::DIV, &range);
                return (return_type, Ok(()));
            }
            OpInfix::Equal => {
                self.emit_u8(op::EQUAL, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::NotEqual => {
                self.emit_u8(op::NOT_EQUAL, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::Greater => {
                self.emit_u8(op::GREATER_THAN, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::GreaterEqual => {
                self.emit_u8(op::GREATER_THAN_EQUAL, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::Less => {
                self.emit_u8(op::LESS_THAN, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::LessEqual => {
                self.emit_u8(op::LESS_THAN_EQUAL, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::LogicOr => {
                self.emit_u8(op::OR, &range);
                return (Type::Bool, Ok(()));
            }
            OpInfix::LogicAnd => {
                self.emit_u8(op::AND, &range);
                return (Type::Bool, Ok(()));
            }
        }
    }

    fn compile_literal(
        &mut self,
        (literal, range): (&ExprLiteral, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> (Type, Result<()>) {
        match literal {
            ExprLiteral::Number(value) => {
                self.emit_constant(Value::Number(*value), &range);
                (Type::Int, Ok(()))
            }
            ExprLiteral::String(string) => {
                let string = allocator.alloc(string);
                self.emit_constant(Value::String(string), &range);
                (Type::String, Ok(()))
            }
            ExprLiteral::Bool(value) => {
                match value {
                    true => self.emit_u8(op::TRUE, &range),
                    false => self.emit_u8(op::FALSE, &range),
                };
                (Type::Bool, Ok(()))
            }
            ExprLiteral::Nil => {
                self.emit_u8(op::NIL, &range);
                (Type::Nil, Ok(()))
            }
        }
    }

    fn begin_scope(&mut self) {
        self.current_compiler.scope_depth += 1;
    }

    fn end_scope(&mut self, span: Range<usize>) {
        self.current_compiler.scope_depth -= 1;
        while !self.current_compiler.locals.is_empty()
            && self.current_compiler.locals.last().unwrap().depth
                > self.current_compiler.scope_depth
        {
            self.current_compiler.locals.pop();
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
