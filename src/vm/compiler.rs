use std::ops::Range;

use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::ast::{
        ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, OpInfix, OpPrefix, Program, Span,
        Statement, StatementExpr, StatementPrint, StatementVar, Type,
    },
};

use super::{chunk::Chunk, op, value::Value};

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self { chunk }
    }
    pub fn compile(&mut self, source: &mut Program, allocator: &mut CeAllocation) {
        loop {
            match source.statements.pop() {
                Some(statement) => self.compile_statement(statement, allocator),
                None => break,
            }
        }

        self.write_byte(op::NIL, 0..0);
        self.write_byte(op::RETURN, 0..0);
    }

    fn compile_statement(
        &mut self,
        statement: (Statement, Range<usize>),
        allocator: &mut CeAllocation,
    ) {
        let (statement, range) = statement;
        match statement {
            Statement::Expression(expr) => self.compile_expression(expr.expr, allocator),
            Statement::Print(print) => self.print_statement((print, range), allocator),
            Statement::Var(var) => self.compile_var((var, range), allocator),
            _ => todo!("statement not implemented"),
        }
    }

    fn print_statement(
        &mut self,
        print: (StatementPrint, Range<usize>),
        allocator: &mut CeAllocation,
    ) {
        let (print, range) = print;
        self.compile_expression(print.value, allocator);
        self.write_byte(op::PRINT, range);
    }

    fn compile_expression(
        &mut self,
        expr: (Expression, Range<usize>),
        allocator: &mut CeAllocation,
    ) {
        let (expr, range) = expr;
        match expr {
            Expression::Prefix(prefix) => self.compile_prefix((prefix, range), allocator),
            Expression::Infix(infix) => self.compile_infix((infix, range), allocator),
            Expression::Literal(value) => self.compile_literal((value, range), allocator),
            _ => todo!("expression not implemented"),
        }
    }

    fn compile_var(&mut self, var: (StatementVar, Range<usize>), allocator: &mut CeAllocation) {
        let (var, range) = var;
        match var.value {
            Some(value) => self.compile_expression(value, allocator),
            None => {
                todo!();
            }
        }
        match var.var.type_ {
            Some(t) => match t {
                Type::String => {
                    let string = allocator.alloc(var.var.name);
                    self.emit_constant(Value::String(string), range);
                }
                _ => todo!(),
            },
            None => {
                todo!();
            }
        }
    }

    fn compile_prefix(
        &mut self,
        prefix: (Box<ExprPrefix>, Range<usize>),
        allocator: &mut CeAllocation,
    ) {
        let (prefix, range) = prefix;
        self.compile_expression(prefix.rt, allocator);
        match prefix.op {
            OpPrefix::Negate => self.write_byte(op::NEG, range),
            OpPrefix::Not => self.write_byte(op::NOT, range),
        }
    }

    fn compile_infix(
        &mut self,
        infix: (Box<ExprInfix>, Range<usize>),
        allocator: &mut CeAllocation,
    ) {
        let (infix, range) = infix;
        self.compile_expression(infix.lhs, allocator);
        self.compile_expression(infix.rhs, allocator);
        match infix.op {
            OpInfix::Add => self.write_byte(op::ADD, range),
            OpInfix::Sub => self.write_byte(op::SUB, range),
            OpInfix::Mul => self.write_byte(op::MUL, range),
            OpInfix::Div => self.write_byte(op::DIV, range),
            OpInfix::Equal => self.write_byte(op::EQUAL, range),
            OpInfix::NotEqual => self.write_byte(op::NOT_EQUAL, range),
            OpInfix::Less => self.write_byte(op::LESS_THAN, range),
            OpInfix::LessEqual => self.write_byte(op::LESS_THAN_EQUAL, range),
            OpInfix::Greater => self.write_byte(op::GREATER_THAN, range),
            OpInfix::GreaterEqual => self.write_byte(op::GREATER_THAN_EQUAL, range),
            OpInfix::LogicOr => self.write_byte(op::OR, range),
            OpInfix::LogicAnd => self.write_byte(op::AND, range),

            _ => panic!("Unknown operator"),
        }
    }

    fn compile_literal(
        &mut self,
        literal: (ExprLiteral, Range<usize>),
        allocator: &mut CeAllocation,
    ) {
        let (literal, range) = literal;
        match literal {
            ExprLiteral::Number(value) => {
                self.emit_constant(Value::Number(value), range);
            }
            ExprLiteral::String(string) => {
                let string = allocator.alloc(string);
                self.emit_constant(Value::String(string), range);
            }
            ExprLiteral::Bool(value) => match value {
                true => self.write_byte(op::TRUE, range),
                false => self.write_byte(op::FALSE, range),
            },
            ExprLiteral::Nil => {
                todo!();
            }
            _ => todo!(),
        }
    }
    fn write_byte(&mut self, byte: u8, span: Span) {
        self.chunk.code.push(byte);
        self.chunk.spans.push(span);
    }

    fn emit_constant(&mut self, value: Value, span: Span) {
        let index = self.chunk.constants.len() as u8;
        self.write_byte(op::CECILE_CONSTANT, span.clone());
        self.write_byte(index, span);
        self.chunk.constants.push(value);
    }
}
