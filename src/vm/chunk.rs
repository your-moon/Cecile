use std::ops::Range;

use crate::allocator::allocation::CeAllocation;
use crate::cc_parser::ast::Span;
use crate::vm::op;
use crate::vm::value::Value;

#[derive(Debug, Clone, Default)]
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

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        match self.code[offset] {
            op::PRINT_LN => self.simple_instruction("PRINT_LN", offset),
            op::LOOP => self.jump_instruction("LOOP", offset),
            op::JUMP => self.jump_instruction("JUMP", offset),
            op::JUMP_IF_FALSE => self.jump_instruction("JUMP_IF_FALSE", offset),
            op::POP => self.simple_instruction("POP", offset),
            op::GET_LOCAL => self.simple_instruction("GET_LOCAL", offset),
            op::SET_LOCAL => self.simple_instruction("SET_LOCAL", offset),
            op::SET_GLOBAL => self.constant_instruction("SET_GLOBAL", offset),
            op::DEFINE_GLOBAL => self.constant_instruction("DEFINE_GLOBAL", offset),
            op::GET_GLOBAL => self.constant_instruction("GET_GLOBAL", offset),
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
            op::TRUE => self.simple_instruction("TRUE", offset),
            op::FALSE => self.simple_instruction("FALSE", offset),
            op::PRINT => self.simple_instruction("PRINT", offset),
            op::NIL => self.simple_instruction("NIL", offset),
            op::RETURN => self.simple_instruction("RETURN", offset),

            _ => {
                println!("Unknown opcode {}", self.code[offset]);
                return offset + 1;
            }
        }
    }

    fn read_u16(&self, offset: usize) -> u16 {
        ((self.code[offset] as u16) << 8) | self.code[offset + 1] as u16
    }

    fn jump_instruction(&self, name: &str, offset: usize) -> usize {
        let jump = self.read_u16(offset + 1);
        println!(
            "{:16} {:4} -> {}",
            name,
            offset,
            offset as isize + 3 + jump as isize
        );
        offset + 3
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
    pub fn emit_u8(&mut self, byte: u8, span: &Span) {
        self.code.push(byte);
        self.spans.push(span.clone());
    }

    pub fn emit_constant(&mut self, value: Value, span: &Span) {
        let index = self.constants.len() as u8;
        self.emit_u8(op::CECILE_CONSTANT, span);
        self.emit_u8(index, span);
        self.constants.push(value);
    }

    pub fn write_constant(&mut self, value: Value, span: &Span) {
        let index = self.constants.len() as u8;
        self.emit_u8(index, span);
        self.constants.push(value);
    }
}
