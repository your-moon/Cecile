use std::{hash::BuildHasherDefault, ops::Range};

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};

use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::ast::{
        ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, OpInfix, OpPrefix, Program, Span,
        Statement, StatementExpr, StatementPrint, StatementVar, Type,
    },
};

use super::{chunk::Chunk, object::StringObject, op, value::Value};
use rustc_hash::FxHasher;

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    globals: HashMap<*mut StringObject, Type, BuildHasherDefault<FxHasher>>,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            globals: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
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
            _ => todo!("statement not implemented"),
        }
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
            Expression::Prefix(prefix) => self.compile_prefix((prefix, range), allocator),
            Expression::Infix(infix) => self.compile_infix((infix, range), allocator),
            Expression::Literal(value) => self.compile_literal((value, range), allocator),
            Expression::Var(var) => self.compile_expression_var((var, range), allocator),
            _ => todo!("expression not implemented"),
        }
    }

    fn compile_statement_var(
        &mut self,
        var: (&StatementVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let (var, range) = var;
        let mut var_type = Type::UnInitialized;
        match &var.value {
            Some(value) => {
                var_type = self.compile_expression(value, allocator);
            }
            None => {
                todo!();
            }
        };
        if &var_type != var.var.type_.as_ref().unwrap() {
            todo!("type mismatch in var declaration");
        }
        match &var.var.type_ {
            Some(t) => match t {
                Type::String => {
                    let name = &var.var.name;
                    let string = allocator.alloc(name);
                    self.globals.insert(string, Type::String);
                    self.write_byte(op::DEFINE_GLOBAL, &range);
                    self.write_constant(Value::String(string), &range);
                }
                Type::Int => {
                    let name = &var.var.name;
                    let string = allocator.alloc(name);
                    self.globals.insert(string, Type::Int);
                    self.write_byte(op::DEFINE_GLOBAL, &range);
                    self.write_constant(Value::String(string), &range);
                }
                _ => todo!("type not implemented"),
            },
            None => {
                todo!();
            }
        }
        return var_type;
    }

    fn compile_expression_var(
        &mut self,
        (prefix, range): (&ExprVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Type {
        let name = allocator.alloc(&prefix.var.name);
        self.write_byte(op::GET_GLOBAL, &range);
        self.write_constant(Value::String(name), &range);
        let type_ = self.globals.get(&name);
        match type_ {
            Some(t) => return t.clone(),
            None => Type::UnInitialized,
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
}
