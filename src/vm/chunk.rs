use arrayvec::ArrayVec;
use std::{collections::HashMap, ops::Index};

use crate::cc_parser::ast::Span;
use crate::vm::op;
use crate::vm::value::Value;

use super::error::{OverflowError, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct ChunkCtx<'a> {
    pub op_code: &'a u8,
    pub value: Option<&'a Value>,
    pub next: usize,
    pub prev: usize,
}

impl<'a> ChunkCtx<'a> {
    pub fn new(op_code: &'a u8, value: Option<&'a Value>) -> Self {
        Self {
            op_code,
            value,
            next: 0,
            prev: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Chunk {
    pub op_codes: Vec<u8>,
    pub constants: ArrayVec<Value, 256>,
    pub spans: VecRun<Span>,
    pos: usize,
    jump: Vec<u8>,
}

impl Chunk {
    pub fn is_end(&self) -> bool {
        self.pos >= self.op_codes.len() - 1
    }
    pub fn current(&self) -> Option<ChunkCtx> {
        if self.pos > self.op_codes.len() - 1 {
            return None;
        }
        match self.op_codes[self.pos] {
            op::CECILE_CONSTANT => {
                let constant_idx = &self.op_codes[self.pos + 1];
                let constant = &self.constants[*constant_idx as usize];
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, Some(constant)))
            }
            op::ADD => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            op::RETURN => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            op::POP => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            op::NIL => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            op::PRINT => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            op::PRINT_LN => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            op::SUB => {
                let current_op_code = &self.op_codes[self.pos];
                Some(ChunkCtx::new(current_op_code, None))
            }
            _ => None,
        }
    }
    pub fn next(&mut self) -> Option<()> {
        if self.is_end() {
            return None;
        }

        match self.op_codes.get(self.pos) {
            Some(&op::CECILE_CONSTANT) => {
                self.pos += 2;
                self.jump.push(2);
                Some(())
            }
            Some(&op::ADD) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(&op::RETURN) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(&op::POP) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(&op::NIL) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(&op::PRINT) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(&op::PRINT_LN) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(&op::SUB) => {
                self.pos += 1;
                self.jump.push(1);
                Some(())
            }
            Some(_) => None,
            None => None,
        }
    }

    pub fn prev(&mut self) -> Option<()> {
        let jump = self.jump.pop();
        match jump {
            Some(jump) => {
                self.pos -= jump as usize;
                Some(())
            }
            None => None,
        }
    }
    pub fn write_u8(&mut self, byte: u8, span: &Span) {
        self.op_codes.push(byte);
        self.spans.push(span.clone());
    }

    /// Writes a constant to the [`Chunk`] and returns its index. If an equal
    /// [`Value`] is already present, then its index is returned instead.
    pub fn write_constant(&mut self, value: Value, span: &Span) -> Result<u8> {
        let idx = match self
            .constants
            .iter()
            .position(|&constant| constant == value)
        {
            Some(idx) => idx,
            None => {
                self.constants
                    .try_push(value)
                    .map_err(|_| (OverflowError::TooManyConstants.into(), span.clone()))?;
                self.constants.len() - 1
            }
        };
        Ok(idx.try_into().expect("constant index overflow"))
    }
    //
    // pub fn constant_folding(&mut self) {
    //     let mut idx = 0;
    //     while idx < self.op_codes.len() {
    //         idx = self.constant_folding_op(idx);
    //     }
    // }
    //
    // pub fn constant_folding_op(&mut self, idx: usize) -> usize {
    //     match self.op_code_names[idx] {
    //         op::ADD | op::SUB | op::MUL | op::DIV => {
    //             let left = if self.op_code_names[idx - 2] == op::CECILE_CONSTANT {
    //                 println!("op: {:?}", self.op_codes[idx]);
    //                 println!("left: {:?}", self.op_codes[idx - 4]);
    //                 let constant_idx = self.op_codes[idx - 3] as usize;
    //                 self.constants[constant_idx]
    //             } else {
    //                 return idx + 1;
    //             };
    //
    //             let right = if self.op_code_names[idx - 1] == op::CECILE_CONSTANT {
    //                 self.constants[self.op_codes[idx - 1] as usize]
    //             } else {
    //                 return idx + 1;
    //             };
    //
    //             let result = match self.op_codes[idx] {
    //                 op::ADD => left.as_number() + right.as_number(),
    //                 op::SUB => left.as_number() - right.as_number(),
    //                 op::MUL => left.as_number() * right.as_number(),
    //                 op::DIV => left.as_number() / right.as_number(),
    //                 _ => unreachable!(),
    //             };
    //             let result = result.into();
    //             let constant_idx = self
    //                 .constants
    //                 .iter()
    //                 .position(|&constant| constant == result)
    //                 .unwrap_or_else(|| {
    //                     if self.constants.len() < self.constants.capacity() {
    //                         self.constants.push(result);
    //                         self.constants.len() - 1
    //                     } else {
    //                         panic!("too many constants")
    //                     }
    //                 });
    //
    //             // Replace the operation with a CONSTANT operation
    //             self.op_codes[idx - 4] = op::CECILE_CONSTANT;
    //             self.op_codes[idx - 3] = constant_idx as u8;
    //             // Remove the second CONSTANT operation and the original operation
    //             self.op_codes.drain(idx - 2..=idx);
    //
    //             idx - 2
    //         }
    //         // op::BUILD_ARRAY
    //         // | op::GET_ARRAY_METHOD
    //         // | op::CECILE_CONSTANT
    //         // | op::GET_LOCAL
    //         // | op::SET_LOCAL
    //         // | op::SET_GLOBAL
    //         // | op::GET_UPVALUE
    //         // | op::SET_UPVALUE
    //         // | op::GET_FIELD
    //         // | op::SET_FIELD
    //         // | op::GET_SUPER
    //         // | op::CALL
    //         // | op::STRUCT
    //         // | op::FIELD
    //         // | op::METHOD => idx + 2,
    //         // op::INVOKE | op::SUPER_INVOKE => idx + 3,
    //         // op::JUMP | op::JUMP_IF_FALSE | op::LOOP => idx + 3,
    //         _ => idx + 1,
    //     }
    // }
    //
    // pub fn constant_propagation(&mut self) {
    //     let mut constants = HashMap::new();
    //     let mut idx = 0;
    //     while idx < self.op_codes.len() {
    //         match self.op_codes[idx] {
    //             op::DEFINE_GLOBAL => {
    //                 if self.op_codes[idx - 2] == op::CECILE_CONSTANT {
    //                     let constant_idx = self.op_codes[idx - 1] as usize;
    //
    //                     let code = self.op_codes[idx + 1];
    //                     let constant = self.constants[code as usize];
    //                     let global_name = unsafe { (*constant.as_object().string).value };
    //                     constants.insert(global_name.to_string(), constant_idx);
    //                     idx += 2;
    //                 } else {
    //                     idx += 3;
    //                 }
    //             }
    //             op::GET_GLOBAL => {
    //                 let code = self.op_codes[idx + 1] as usize;
    //                 let constant = self.constants[code];
    //                 let global_name = unsafe { (*constant.as_object().string).value };
    //                 if let Some(constant_idx) = constants.get(&global_name.to_string()) {
    //                     // Replace the GET_GLOBAL operation with a CONSTANT operation
    //                     self.op_codes[idx] = op::CECILE_CONSTANT;
    //                     self.op_codes[idx + 1] = *constant_idx as u8;
    //                 }
    //                 idx += 2;
    //             }
    //             op::BUILD_ARRAY
    //             | op::GET_ARRAY_METHOD
    //             | op::CECILE_CONSTANT
    //             | op::GET_LOCAL
    //             | op::SET_LOCAL
    //             | op::SET_GLOBAL
    //             | op::GET_UPVALUE
    //             | op::SET_UPVALUE
    //             | op::GET_FIELD
    //             | op::SET_FIELD
    //             | op::GET_SUPER
    //             | op::CALL
    //             | op::STRUCT
    //             | op::FIELD
    //             | op::METHOD => idx += 2,
    //             op::INVOKE | op::SUPER_INVOKE => idx += 3,
    //             op::JUMP | op::JUMP_IF_FALSE | op::LOOP => idx += 3,
    //             _ => idx += 1,
    //         }
    //     }
    // }
    //
    pub fn debug(&self, name: &str) {
        eprintln!("== {name} ==");
        let mut idx = 0;
        while idx < self.op_codes.len() {
            idx = self.debug_op(idx);
        }
    }

    pub fn debug_op(&self, idx: usize) -> usize {
        eprint!("{idx:04} ");
        match self.op_codes[idx] {
            op::BUILD_ARRAY => self.debug_op_byte("BUILD_ARRAY", idx),
            op::BINARY_GETELEM => self.debug_op_simple("BINARY_GETELEM", idx),
            op::ARRAY_ELEM_ASSIGN => self.debug_op_simple("ARRAY_ELEM_ASSIGN", idx),
            op::GET_ARRAY_METHOD => self.debug_op_byte("GET_ARRAY_METHOD", idx),
            op::GET_COPY_ARRAY => self.debug_op_simple("GET_COPY_ARRAY", idx),
            op::CECILE_CONSTANT => self.debug_op_constant("OP_CONSTANT", idx),
            op::NIL => self.debug_op_simple("OP_NIL", idx),
            op::TRUE => self.debug_op_simple("OP_TRUE", idx),
            op::FALSE => self.debug_op_simple("OP_FALSE", idx),
            op::POP => self.debug_op_simple("OP_POP", idx),
            op::GET_LOCAL => self.debug_op_byte("OP_GET_LOCAL", idx),
            op::SET_LOCAL => self.debug_op_byte("OP_SET_LOCAL", idx),
            op::GET_GLOBAL => self.debug_op_constant("OP_GET_GLOBAL", idx),
            op::DEFINE_GLOBAL => self.debug_op_constant("OP_DEFINE_GLOBAL", idx),
            op::SET_GLOBAL => self.debug_op_constant("OP_SET_GLOBAL", idx),
            op::GET_UPVALUE => self.debug_op_byte("OP_GET_UPVALUE", idx),
            op::SET_UPVALUE => self.debug_op_byte("OP_SET_UPVALUE", idx),
            op::GET_FIELD => self.debug_op_constant("OP_GET_FIELD", idx),
            op::SET_FIELD => self.debug_op_constant("OP_SET_FIELD", idx),
            op::GET_SUPER => self.debug_op_constant("OP_GET_SUPER", idx),
            op::EQUAL => self.debug_op_simple("OP_EQUAL", idx),
            op::NOT_EQUAL => self.debug_op_simple("OP_NOT_EQUAL", idx),
            op::GREATER_THAN => self.debug_op_simple("OP_GREATER", idx),
            op::GREATER_THAN_EQUAL => self.debug_op_simple("OP_GREATER_EQUAL", idx),
            op::LESS_THAN => self.debug_op_simple("OP_LESS", idx),
            op::LESS_THAN_EQUAL => self.debug_op_simple("OP_LESS_EQUAL", idx),
            op::ADD => self.debug_op_simple("OP_ADD", idx),
            op::MODULO => self.debug_op_simple("OP_MODULO", idx),
            op::CONCAT => self.debug_op_simple("OP_CONCAT", idx),
            op::SUB => self.debug_op_simple("OP_SUBTRACT", idx),
            op::MUL => self.debug_op_simple("OP_MULTIPLY", idx),
            op::DIV => self.debug_op_simple("OP_DIVIDE", idx),
            op::NOT => self.debug_op_simple("OP_NOT", idx),
            op::NEG => self.debug_op_simple("OP_NEGATE", idx),
            op::PRINT => self.debug_op_simple("OP_PRINT", idx),
            op::PRINT_LN => self.debug_op_simple("OP_PRINT_LN", idx),
            op::JUMP => self.debug_op_jump("OP_JUMP", idx, true),
            op::JUMP_IF_FALSE => self.debug_op_jump("OP_JUMP_IF_FALSE", idx, true),
            op::LOOP => self.debug_op_jump("OP_LOOP", idx, false),
            op::CALL => self.debug_op_byte("OP_CALL", idx),
            op::INVOKE => self.debug_op_invoke("OP_INVOKE", idx),
            op::SUPER_INVOKE => self.debug_op_invoke("OP_SUPER_INVOKE", idx),
            op::CLOSURE => {
                let mut idx = idx + 1;
                let constant_idx = self.op_codes[idx];
                let constant = &self.constants[constant_idx as usize];
                eprintln!(
                    "{name:16} {constant_idx:>4} '{constant}'",
                    name = "OP_CLOSURE"
                );

                let function = unsafe { constant.as_object().function };
                for _ in 0..unsafe { (*function).upvalue_count } {
                    let offset = idx;

                    idx += 1;
                    let is_local = self.op_codes[idx];
                    let label = if is_local == 0 { "upvalue" } else { "local" };

                    idx += 1;
                    let upvalue_idx = self.op_codes[idx];

                    eprintln!("{offset:04} |                     {label} {upvalue_idx}");
                }

                idx + 1
            }
            op::CLOSE_UPVALUE => self.debug_op_simple("OP_CLOSE_UPVALUE", idx),
            op::RETURN => self.debug_op_simple("OP_RETURN", idx),
            op::STRUCT => self.debug_op_constant("OP_STRUCT", idx),
            op::FIELD => self.debug_op_constant("FIELD", idx),
            op::INHERIT => self.debug_op_simple("OP_INHERIT", idx),
            op::METHOD => self.debug_op_constant("OP_METHOD", idx),
            byte => self.debug_op_simple(&format!("OP_UNKNOWN({byte:#X})"), idx),
        }
    }

    fn debug_op_byte(&self, name: &str, idx: usize) -> usize {
        let byte = self.op_codes[idx + 1];
        eprintln!("{name:16} {byte:>4}",);
        idx + 2
    }

    fn debug_op_simple(&self, name: &str, idx: usize) -> usize {
        eprintln!("{name}");
        idx + 1
    }

    fn debug_op_constant(&self, name: &str, idx: usize) -> usize {
        let constant_idx = self.op_codes[idx + 1];
        let constant = &self.constants[constant_idx as usize];
        eprintln!("{name:16} {constant_idx:>4} '{constant}'");
        idx + 2
    }

    fn debug_op_invoke(&self, name: &str, idx: usize) -> usize {
        let constant_idx = self.op_codes[idx + 1];
        let constant = &self.constants[constant_idx as usize];
        let arg_count = self.op_codes[idx + 2];
        eprintln!("{name:16} ({arg_count} args) {constant_idx:>4} '{constant}'");
        idx + 3
    }

    fn debug_op_jump(&self, name: &str, idx: usize, is_forward: bool) -> usize {
        let to_offset = u16::from_le_bytes([self.op_codes[idx + 1], self.op_codes[idx + 2]]);
        let offset_sign = if is_forward { 1 } else { -1 };
        // The +3 is to account for the 3 byte jump instruction.
        let to_idx = (idx as isize) + (to_offset as isize) * offset_sign + 3;
        eprintln!("{name:16} {idx:>4} -> {to_idx}");
        idx + 3
    }

    pub fn clear(&mut self) {
        self.op_codes.clear();
        self.constants.clear();
        self.spans.clear();
    }
}

/// Run-length encoded [`Vec`]. Useful for storing data with a lot of contiguous
/// runs of the same value.
#[derive(Debug, Default)]
pub struct VecRun<T> {
    values: Vec<Run<T>>,
}

impl<T: Eq> VecRun<T> {
    fn push(&mut self, value: T) {
        match self.values.last_mut() {
            Some(run) if run.value == value && run.count < u8::MAX => {
                run.count += 1;
            }
            _ => self.values.push(Run { value, count: 1 }),
        };
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl<T> Index<usize> for VecRun<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut count = index;
        for run in &self.values {
            match count.checked_sub(run.count as usize) {
                Some(remaining) => count = remaining,
                None => return &run.value,
            }
        }
        panic!("index out of bounds");
    }
}

#[derive(Debug)]
struct Run<T> {
    value: T,
    count: u8,
}

#[cfg(test)]

mod tests {
    #[test]
    fn test_opcode_iterator() {
        use super::*;
        let mut op_codes = vec![
            op::CECILE_CONSTANT,
            0,
            op::CECILE_CONSTANT,
            1,
            op::ADD,
            op::RETURN,
        ];
        let mut constants = ArrayVec::new();
        constants.push(1.0.into());
        constants.push(2.0.into());
        let mut iterator = Chunk {
            op_codes,
            constants,
            ..Default::default()
        };

        // let current = iterator.current();
        // assert_eq!(
        //     Some(ChunkCtx::new(&op::CECILE_CONSTANT, Some(&1.0.into()))),
        //     current
        // );
        // assert_eq!(0, iterator.pos);
        //
        // let is_iter = iterator.next();
        // assert_eq!(Some(()), is_iter);
        // assert_eq!(2, iterator.pos);
        // assert_eq!(Some(2), iterator.jump.last().copied());
        //
        // let current = iterator.current();
        // assert_eq!(
        //     Some(ChunkCtx::new(&op::CECILE_CONSTANT, Some(&2.0.into()))),
        //     current
        // );
        //
        // let is_iter = iterator.next();
        // assert_eq!(Some(()), is_iter);
        // assert_eq!(4, iterator.pos);
        // assert_eq!(Some(2), iterator.jump.last().copied());
        //
        // let current = iterator.current();
        // assert_eq!(Some(ChunkCtx::new(&op::ADD, None)), current);
        //
        // let prev = iterator.peek_prev(2);
        // assert_eq!(
        //     Some(ChunkCtx::new(&op::CECILE_CONSTANT, Some(&2.0.into()))),
        //     prev
        // );
        //
        // let is_iter = iterator.next();
        // assert_eq!(Some(()), is_iter);
        // assert_eq!(5, iterator.pos);
        // assert_eq!(Some(1), iterator.jump.last().copied());
        //
        // let current = iterator.current();
        // assert_eq!(Some((&op::RETURN, None)), current);
        //
        // let is_end = iterator.is_end();
        // assert_eq!(true, is_end);
        // // // it should not go next pos because it is the end of the op_codes
        // let is_iter = iterator.next();
        // assert_eq!(None, is_iter);
        // assert_eq!(5, iterator.pos);
        // assert_eq!(Some(1), iterator.jump.last().copied());
        //
        // let current = iterator.current();
        // assert_eq!(Some((&op::RETURN, None)), current);
        //
        // assert_eq!(true, iterator.is_end());
        //
        // assert_eq!(5, iterator.pos);
        // assert_eq!(Some(1), iterator.jump.last().copied());
        // let is_iter = iterator.prev();
        //
        // assert_eq!(Some(()), is_iter);
        //
        // let current = iterator.current();
        // assert_eq!(Some((&op::ADD, None)), current);
        //
        // assert_eq!(4, iterator.pos);
        // assert_eq!(Some(2), iterator.jump.last().copied());
        // let is_iter = iterator.prev();
        //
        // assert_eq!(Some(()), is_iter);
        // assert_eq!(2, iterator.pos);
        // assert_eq!(Some(2), iterator.jump.last().copied());
        //
        // let current = iterator.current();
        // assert_eq!(Some((&op::CECILE_CONSTANT, Some(&2.0.into()))), current);
        //
        // assert_eq!(2, iterator.pos);
        // assert_eq!(Some(2), iterator.jump.last().copied());
        // let is_iter = iterator.prev();
        //
        // assert_eq!(Some(()), is_iter);
        // assert_eq!(0, iterator.pos);
        // assert_eq!(None, iterator.jump.last());
        //
        // let current = iterator.current();
        // assert_eq!(Some((&op::CECILE_CONSTANT, Some(&1.0.into()))), current);
    }
}
