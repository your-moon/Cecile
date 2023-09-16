use std::ops::Range;

use crate::cc_parser::ast::{
    ExprInfix, ExprLiteral, Expression, OpInfix, Program, Span, Statement, StatementExpr,
};
use crate::vm::value::Value;

use super::op;

#[derive(Debug, Clone)]
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

    fn compile_literal(&mut self, literal: (ExprLiteral, Range<usize>)) {
        let (literal, range) = literal;
        match literal {
            ExprLiteral::Number(value) => {
                self.emit_constant(Value::Number(value), range);
            }
            ExprLiteral::String(value) => {
                todo!();
            }
            ExprLiteral::Bool(value) => {
                todo!();
            }
            ExprLiteral::Nil => {
                todo!();
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
        self.write_byte(op::CECILE_CONSTANT, span.clone());
        self.write_byte(index, span);
        self.constants.push(value);
    }

    pub fn disassemble(&self) {
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        match self.code[offset] {
            op::ADD => self.simple_instruction("ADD", offset),
            op::SUB => self.simple_instruction("SUB", offset),
            op::MUL => self.simple_instruction("MUL", offset),
            op::DIV => self.simple_instruction("DIV", offset),
            op::EQUAL => self.simple_instruction("EQUAL", offset),
            op::NOT_EQUAL => self.simple_instruction("NOT_EQUAL", offset),
            op::LESS_THAN => self.simple_instruction("LESS_THAN", offset),
            op::LESS_THAN_EQUAL => self.simple_instruction("LESS_THAN_EQUAL", offset),
            op::GREATER_THAN => self.simple_instruction("GREATER_THAN", offset),
            op::GREATER_THAN_EQUAL => self.simple_instruction("GREATER_THAN_EQUAL", offset),
            op::AND => self.simple_instruction("AND", offset),
            op::OR => self.simple_instruction("OR", offset),
            op::NOT => self.simple_instruction("NOT", offset),
            op::NEG => self.simple_instruction("NEG", offset),
            op::CECILE_CONSTANT => self.constant_instruction("CECILE_CONSTANT", offset),
            _ => {
                println!("Unknown opcode {}", self.code[offset]);
                return offset + 1;
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{:16} {:4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 2
    }
}
