use std::{fmt::Display, hash::BuildHasherDefault, ops::Range};

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};

use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::ast::{
        ExprAssign, ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, OpInfix, OpPrefix,
        Program, Span, Statement, StatementBlock, StatementExpr, StatementPrint, StatementVar,
        Type,
    },
};

use super::{chunk::Chunk, object::StringObject, op, value::Value};
use rustc_hash::FxHasher;

#[derive(Debug, Default)]
struct Local {
    name: String,
    depth: usize,
    is_initialized: bool,
}

impl Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{name: {}, depth: {}, is_initialized: {}}}",
            self.name, self.depth, self.is_initialized
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

        self.write_byte(op::NIL, &(0..0));
        self.write_byte(op::RETURN, &(0..0));
    }

    fn compile_statement(
        &mut self,
        (statement, range): &(Statement, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        match statement {
            Statement::Expression(expr) => self.compile_expression(&expr.expr, allocator),
            Statement::Print(print) => self.print_statement((print, range), allocator),
            Statement::Var(var) => self.compile_statement_var((var, range), allocator),
            Statement::Block(block) => self.compile_statement_block((block, range), allocator),
            _ => todo!("statement not implemented"),
        }
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

    fn print_statement(
        &mut self,
        print: (&StatementPrint, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (print, range) = print;
        let expr_type = self.compile_expression(&print.value, allocator);
        self.write_byte(op::PRINT, range);
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
            self.write_byte(op::SET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
        } else {
            let name = allocator.alloc(&assign.lhs.name);
            self.write_byte(op::SET_GLOBAL, &range);
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
                        self.write_byte(op::DEFINE_GLOBAL, &range);
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
                    self.write_byte(op::DEFINE_GLOBAL, &range);
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
            self.write_byte(op::GET_LOCAL, &range);
            self.write_constant(Value::Number(local_index.into()), &range);
            let type_ = self.globals.get(&*name);
            match type_ {
                Some(t) => return t.clone(),
                None => Type::UnInitialized,
            }
        } else {
            let string = allocator.alloc(name);
            self.write_byte(op::GET_GLOBAL, &range);
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
            OpPrefix::Negate => self.write_byte(op::NEG, &range),
            OpPrefix::Not => self.write_byte(op::NOT, &range),
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
        // absolute litiral
        let rhs_type = self.compile_expression(&(infix.rhs), allocator);

        if lhs_type != rhs_type {
            todo!("type mismatch");
        }

        match infix.op {
            OpInfix::Add => self.write_byte(op::ADD, &range),
            OpInfix::Sub => self.write_byte(op::SUB, &range),
            OpInfix::Mul => self.write_byte(op::MUL, &range),
            OpInfix::Div => self.write_byte(op::DIV, &range),
            OpInfix::Equal => self.write_byte(op::EQUAL, &range),
            OpInfix::NotEqual => self.write_byte(op::NOT_EQUAL, &range),
            OpInfix::Less => self.write_byte(op::LESS_THAN, &range),
            OpInfix::LessEqual => self.write_byte(op::LESS_THAN_EQUAL, &range),
            OpInfix::Greater => self.write_byte(op::GREATER_THAN, &range),
            OpInfix::GreaterEqual => self.write_byte(op::GREATER_THAN_EQUAL, &range),
            OpInfix::LogicOr => self.write_byte(op::OR, &range),
            OpInfix::LogicAnd => self.write_byte(op::AND, &range),

            _ => panic!("Unknown operator"),
        };
        return lhs_type;
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
                    true => self.write_byte(op::TRUE, &range),
                    false => self.write_byte(op::FALSE, &range),
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
            self.write_byte(op::POP, &span);
        }
    }

    fn declare_local(&mut self, name: &str, is_initialized: &Type, span: &Span) {
        let is_initialized = match is_initialized {
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

    fn write_byte(&mut self, byte: u8, span: &Span) {
        self.chunk.code.push(byte);
        self.chunk.spans.push(span.clone());
    }

    fn emit_constant(&mut self, value: Value, span: &Span) {
        let index = self.chunk.constants.len() as u8;
        self.write_byte(op::CECILE_CONSTANT, span);
        self.write_byte(index, span);
        self.chunk.constants.push(value);
    }

    fn write_constant(&mut self, value: Value, span: &Span) {
        let index = self.chunk.constants.len() as u8;
        self.write_byte(index, span);
        self.chunk.constants.push(value);
    }

    fn resolve_local(&mut self, name: &str, span: &Span) -> Option<u8> {
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
