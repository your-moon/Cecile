use std::{fmt::Display, hash::BuildHasherDefault, ops::Range};

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};

use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::ast::{
        ExprAssign, ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, OpInfix, OpPrefix,
        Program, Span, Statement, StatementBlock, StatementExpr, StatementFor, StatementIf,
        StatementPrint, StatementPrintLn, StatementVar, StatementWhile, Type,
    },
};

use super::{chunk::Chunk, object::StringObject, op, value::Value};
use rustc_hash::FxHasher;

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

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    pub globals: HashMap<String, Type, BuildHasherDefault<FxHasher>>,
    locals: Vec<Local>,
    scope_depth: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            globals: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            locals: Vec::new(),
            scope_depth: 0,
        }
    }
    pub fn compile(&mut self, source: &mut Program, allocator: &mut CeAllocation) {
        for statement in &source.statements {
            self.compile_statement(statement, allocator);
        }

        self.emit_u8(op::NIL, &(0..0));
        self.emit_u8(op::RETURN, &(0..0));
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
            _ => todo!("statement not implemented"),
        }
    }

    fn compile_statement_for(
        &mut self,
        (for_, range): (&StatementFor, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        self.begin_scope();
        if let Some(init) = &for_.init {
            self.compile_statement(&init, allocator);
        }
        let loop_start = self.chunk.code.len();
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
        return Type::UnInitialized;
    }

    fn compile_statement_while(
        &mut self,
        (while_, range): (&StatementWhile, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let loop_start = self.chunk.code.len();
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
        let offset = self.chunk.code.len() - loop_start + 2;
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
            _ => todo!("expression not implemented"),
        }
    }

    fn compile_assign(
        &mut self,
        (assign, range): (&Box<ExprAssign>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let var_type = self.compile_expression(&assign.rhs, allocator);
        if let Some(local_index) = self.resolve_local(&assign.lhs.name, &range) {
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
                        self.globals.insert(name.clone(), t.clone());
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
                    self.globals.insert(name.clone(), var_type.clone());
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
        if let Some(local_index) = self.resolve_local(&prefix.var.name, &range) {
            self.emit_u8(op::GET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
            self.locals[local_index as usize].type_.clone()
        } else {
            let string = allocator.alloc(name);
            self.emit_u8(op::GET_GLOBAL, &range);
            self.write_constant(Value::String(string), &range);
            let type_ = self.globals.get(&*name);
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
        // absolute litiral
        let lhs_type = self.compile_expression(&(infix.lhs), allocator);
        println!("lhs_type: {:?}", lhs_type);
        // absolute litiral
        let rhs_type = self.compile_expression(&(infix.rhs), allocator);
        println!("rhs_type: {:?}", rhs_type);
        //if lhs_type: String rhs_type: String => concat string command will be called
        //if lhs_type: Int  rhs_type: Int  => add int command will be called

        if lhs_type != rhs_type {
            todo!("type mismatch");
        }

        match infix.op {
            OpInfix::Add => {
                self.emit_u8(op::ADD, &range);
                return lhs_type;
            }
            OpInfix::Sub => {
                self.emit_u8(op::SUB, &range);
                return lhs_type;
            }
            OpInfix::Mul => {
                self.emit_u8(op::MUL, &range);
                return lhs_type;
            }
            OpInfix::Div => {
                self.emit_u8(op::DIV, &range);
                return lhs_type;
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
        literal: (&ExprLiteral, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (literal, range) = literal;
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
                todo!();
            }
            _ => todo!(),
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self, span: Range<usize>, allocator: &mut CeAllocation) {
        self.scope_depth -= 1;
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            let local = self.locals.pop().unwrap();
            self.emit_u8(op::POP, &span);
        }
    }

    fn declare_local(&mut self, name: &str, type_: &Type, span: &Span) {
        let is_initialized = match type_ {
            Type::UnInitialized => false,
            _ => true,
        };
        for local in self.locals.iter().rev() {
            if local.depth != self.scope_depth {
                break;
            }

            if local.name == name {
                todo!("Already declared variable in this scope");
            }
        }

        let local = Local {
            name: name.to_string(),
            type_: type_.clone(),
            depth: self.scope_depth,
            is_initialized,
        };
        self.locals.push(local);
    }

    fn define_local(&mut self) {
        let local = self
            .locals
            .last_mut()
            .expect("Define local called without locals");
        local.is_initialized = true;
    }

    fn is_global(&self) -> bool {
        self.scope_depth == 0
    }

    fn emit_jump(&mut self, instruction: u8, span: &Span) -> usize {
        self.emit_u8(instruction, span);
        self.emit_u8(0xff, span);
        self.emit_u8(0xff, span);
        self.chunk.code.len() - 2
    }

    fn patch_jump(&mut self, offset_idx: usize, span: &Span) {
        let offset = self.chunk.code.len() - offset_idx - 2;
        if offset > u16::MAX as usize {
            todo!("Too much code to jump over");
        }
        self.chunk.code[offset_idx] = (offset >> 8) as u8;
        self.chunk.code[offset_idx + 1] = offset as u8;
    }

    fn emit_u8(&mut self, byte: u8, span: &Span) {
        self.chunk.code.push(byte);
        self.chunk.spans.push(span.clone());
    }

    fn emit_constant(&mut self, value: Value, span: &Span) {
        let index = self.chunk.constants.len() as u8;
        self.emit_u8(op::CECILE_CONSTANT, span);
        self.emit_u8(index, span);
        self.chunk.constants.push(value);
    }

    fn write_constant(&mut self, value: Value, span: &Span) {
        let index = self.chunk.constants.len() as u8;
        self.emit_u8(index, span);
        self.chunk.constants.push(value);
    }

    fn resolve_local(&mut self, name: &str, span: &Span) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                println!("local: {}", local);
                if !local.is_initialized {
                    todo!("Can't read local variable in its own initializer");
                }
                return Some(i as u8);
            }
        }
        None
    }
}
