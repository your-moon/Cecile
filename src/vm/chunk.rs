use std::ops::Range;

use crate::cc_parser::ast::{
    ExprInfix, ExprLiteral, Expression, OpInfix, Program, Span, Statement, StatementExpr,
};
use crate::vm::value::Value;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub spans: Vec<Span>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            spans: Vec::new(),
        }
    }

    pub fn compile(&mut self, source: &mut Program) {
        loop {
            match source.statements.pop() {
                Some(statement) => self.compile_statement(statement),
                None => break,
            }
        }
    }

    fn compile_statement(&mut self, statement: (Statement, Range<usize>)) {
        let (statement, range) = statement;
        match statement {
            Statement::Expression(expr) => self.compile_expression(expr.expr),
            _ => todo!(),
        }
    }

    fn compile_expression(&mut self, expr: (Expression, Range<usize>)) {
        let (expr, range) = expr;
        match expr {
            Expression::Infix(infix) => self.compile_infix((infix, range)),
            Expression::Literal(value) => self.compile_literal((value, range)),
            _ => todo!(),
        }
    }

    fn compile_infix(&mut self, infix: (Box<ExprInfix>, Range<usize>)) {
        let (infix, range) = infix;
        self.compile_expression(infix.lhs);
        self.compile_expression(infix.rhs);
        match infix.op {
            OpInfix::Add => self.write_byte(0x01, range),
            OpInfix::Sub => self.write_byte(0x02, range),
            OpInfix::Mul => self.write_byte(0x03, range),
            OpInfix::Div => self.write_byte(0x04, range),
            OpInfix::Equal => self.write_byte(0x05, range),
            OpInfix::NotEqual => self.write_byte(0x06, range),
            OpInfix::Less => self.write_byte(0x07, range),
            OpInfix::LessEqual => self.write_byte(0x08, range),
            OpInfix::Greater => self.write_byte(0x09, range),
            OpInfix::GreaterEqual => self.write_byte(0x0a, range),
            OpInfix::LogicOr => self.write_byte(0x0b, range),
            OpInfix::LogicAnd => self.write_byte(0x0c, range),

            _ => panic!("Unknown operator"),
        }
    }

    fn compile_literal(&mut self, literal: (ExprLiteral, Range<usize>)) {
        let (literal, range) = literal;
        match literal {
            ExprLiteral::Number(value) => {
                self.emit_constant(Value::Number(value), range);
            }
            ExprLiteral::String(value) => {
                self.emit_constant(Value::Number(123.0), range);
            }
            ExprLiteral::Bool(value) => {
                self.emit_constant(Value::Number(0.0), range);
            }
            ExprLiteral::Nil => {
                self.emit_constant(Value::Number(0.0), range);
            }
            _ => todo!(),
        }
    }

    fn write_byte(&mut self, byte: u8, span: Span) {
        self.code.push(byte);
        self.spans.push(span);
    }

    fn emit_constant(&mut self, value: Value, span: Span) {
        let index = self.constants.len() as u8;
        self.write_byte(index, span);
        self.constants.push(value);
    }
}
