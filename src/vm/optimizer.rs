use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::cc_parser::ast::Span;

use super::{
    chunk::{Chunk, VecRun},
    object::ObjectFunction,
    op,
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: *mut ObjectFunction,
    pub upvalues: Vec<OptUpvalue>,
}

#[derive(Debug, Clone)]
pub struct OptUpvalue {
    pub index: u8,
    pub is_local: bool,
}
impl OptUpvalue {
    pub fn new(index: u8, is_local: bool) -> Self {
        Self { index, is_local }
    }
}

impl Closure {
    pub fn new(function: *mut ObjectFunction, upvalues: Vec<OptUpvalue>) -> Self {
        Self { function, upvalues }
    }
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Constant(u8),
    Closure(Closure),
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Neg,
    Not,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    And,
    Or,
    True,
    False,
    Nil,
    Return,
    Pop,
    Print,
    PrintLn,
    Concat,
    CloseUpvalue,
    Inherit,
    GetLocal { index: u8 },
    SetLocal { index: u8 },
    GetUpvalue { index: u8 },
    SetUpvalue { index: u8 },
    GetGlobal { index: u8 },
    SetGlobal { index: u8 },
    DefineGlobal { index: u8 },
    GetField { index: u8 },
    SetField { index: u8 },
    BuildArray { size: u8 },
    GetArrayMethod { index: u8 },
    ArrayElemAssign,
    BinaryGetelem,
    Call { arg_count: u8 },
    Method { index: u8 },
    Struct { index: u8 },
    Field { index: u8 },
    Invoke { arg_count: u8 },
    SuperInvoke { arg_count: u8 },
    GetSuper { index: u8 },
    Jump { from: u8, to: u8 },
    JumpIfFalse { from: u8, to: u8 },
    Loop { from: u8, to: u8 },
}

#[derive(Debug)]
pub struct CountedChunk {
    pub op_codes: Vec<Opcode>,
    pub constants: ArrayVec<Value, 256>,
    pub spans: VecRun<Span>,
    pub prop_constants: HashMap<String, u8>,
}

impl CountedChunk {
    pub fn new(constants: ArrayVec<Value, 256>, spans: VecRun<Span>) -> Self {
        CountedChunk {
            op_codes: Vec::new(),
            constants,
            prop_constants: HashMap::new(),
            spans,
        }
    }

    pub fn constant_folding(&mut self) {
        let mut idx = 0;
        while idx < self.op_codes.len() {
            idx = self.constant_folding_op(idx);
        }
    }

    pub fn constant_folding_op(&mut self, idx: usize) -> usize {
        match self.op_codes[idx] {
            Opcode::Add | Opcode::Sub => {
                let left = if let Opcode::Constant(left) = &self.op_codes[idx - 2] {
                    let const_idx = *left;
                    self.constants[const_idx as usize].clone()
                } else {
                    return idx + 1;
                };

                let right = if let Opcode::Constant(right) = &self.op_codes[idx - 1] {
                    let const_idx = *right;
                    self.constants[const_idx as usize].clone()
                } else {
                    return idx + 1;
                };

                let result = match self.op_codes[idx] {
                    Opcode::Add => left.as_number() + right.as_number(),
                    Opcode::Sub => left.as_number() - right.as_number(),
                    _ => unreachable!(),
                };

                let result = result.into();

                let const_idx = self
                    .constants
                    .iter()
                    .position(|c| *c == result)
                    .unwrap_or_else(|| {
                        if self.constants.len() < self.constants.capacity() {
                            self.constants.push(result);
                            self.constants.len() - 1
                        } else {
                            panic!("too many constants")
                        }
                    });

                let opcode = Opcode::Constant(const_idx as u8);
                self.op_codes[idx - 2] = opcode;
                self.op_codes.drain(idx - 1..=idx);
                self.spans.drain(idx - 1..=idx);

                idx - 2
            }
            _ => idx + 1,
        }
    }

    pub fn constant_propagation(&mut self) {
        let mut idx = 0;
        while idx < self.op_codes.len() {
            idx = self.constant_propagation_op(idx);
        }
    }

    pub fn constant_propagation_op(&mut self, idx: usize) -> usize {
        match self.op_codes[idx] {
            Opcode::Add => {
                let left = self.op_codes[idx - 2].clone();
                let right = self.op_codes[idx - 1].clone();

                match (left, right) {
                    (Opcode::GetGlobal { index }, Opcode::Constant(right))
                    | (Opcode::Constant(right), Opcode::GetGlobal { index }) => {
                        let right_constant = self.constants[right as usize];
                        let global_value = self.constants[index as usize];
                        let global_name = unsafe { (*global_value.as_object().string).value };
                        if let Some(constant_idx) =
                            self.prop_constants.get(&global_name.to_string())
                        {
                            let constant = self.constants[*constant_idx as usize];
                            let result = constant.as_number() + right_constant.as_number();
                            let result = result.into();
                            let const_idx = self
                                .constants
                                .iter()
                                .position(|c| *c == result)
                                .unwrap_or_else(|| {
                                    if self.constants.len() < self.constants.capacity() {
                                        self.constants.push(result);
                                        self.constants.len() - 1
                                    } else {
                                        panic!("too many constants")
                                    }
                                });
                            let opcode = Opcode::Constant(const_idx as u8);
                            self.op_codes[idx - 2] = opcode;
                            self.op_codes.drain(idx - 1..=idx);
                            self.spans.drain(idx - 1..=idx);
                            return idx - 2;
                        }
                        return idx + 1;
                    }
                    _ => return idx + 1,
                };
            }
            Opcode::DefineGlobal { index } => {
                if let Opcode::Constant(constant_idx) = &self.op_codes[idx - 1] {
                    let constant = self.constants[index as usize];
                    let global_name = unsafe { (*constant.as_object().string).value };
                    self.prop_constants
                        .insert(global_name.to_string(), *constant_idx);
                } else {
                }
                idx + 1
            }
            Opcode::SetGlobal { index } => {
                if let Opcode::Constant(constant_idx) = &self.op_codes[idx - 1] {
                    let constant = self.constants[index as usize];
                    let global_name = unsafe { (*constant.as_object().string).value };
                    self.prop_constants
                        .insert(global_name.to_string(), *constant_idx);
                    idx + 1
                } else {
                    idx + 1
                }
            }
            //
            _ => idx + 1,
        }
    }

    pub fn build_opcode_u8(&mut self) -> Vec<u8> {
        let mut op_codes = Vec::new();
        for opcode in &self.op_codes {
            match opcode {
                Opcode::Constant(constant_idx) => {
                    op_codes.push(op::CECILE_CONSTANT);
                    op_codes.push(*constant_idx);
                }
                Opcode::Closure(closure) => {
                    op_codes.push(op::CLOSURE);
                    op_codes.push(self.constants.len() as u8);
                    self.constants.push((closure.function).into());
                    for upvalue in &closure.upvalues {
                        op_codes.push(if upvalue.is_local { 1 } else { 0 });
                        op_codes.push(upvalue.index);
                    }
                }
                Opcode::Add => op_codes.push(op::ADD),
                Opcode::Sub => op_codes.push(op::SUB),
                Opcode::Mul => op_codes.push(op::MUL),
                Opcode::Div => op_codes.push(op::DIV),
                Opcode::Modulo => op_codes.push(op::MODULO),
                Opcode::Neg => op_codes.push(op::NEG),
                Opcode::Not => op_codes.push(op::NOT),
                Opcode::Equal => op_codes.push(op::EQUAL),
                Opcode::NotEqual => op_codes.push(op::NOT_EQUAL),
                Opcode::LessThan => op_codes.push(op::LESS_THAN),
                Opcode::LessThanEqual => op_codes.push(op::LESS_THAN_EQUAL),
                Opcode::GreaterThan => op_codes.push(op::GREATER_THAN),
                Opcode::GreaterThanEqual => op_codes.push(op::GREATER_THAN_EQUAL),
                Opcode::And => op_codes.push(op::AND),
                Opcode::Or => op_codes.push(op::OR),
                Opcode::True => op_codes.push(op::TRUE),
                Opcode::False => op_codes.push(op::FALSE),
                Opcode::Nil => op_codes.push(op::NIL),
                Opcode::Return => op_codes.push(op::RETURN),
                Opcode::Pop => op_codes.push(op::POP),
                Opcode::Print => op_codes.push(op::PRINT),
                Opcode::PrintLn => op_codes.push(op::PRINT_LN),
                Opcode::Concat => op_codes.push(op::CONCAT),
                Opcode::CloseUpvalue => op_codes.push(op::CLOSE_UPVALUE),
                Opcode::Inherit => op_codes.push(op::INHERIT),
                Opcode::GetLocal { index } => {
                    op_codes.push(op::GET_LOCAL);
                    op_codes.push(*index);
                }
                Opcode::SetLocal { index } => {
                    op_codes.push(op::SET_LOCAL);
                    op_codes.push(*index);
                }
                Opcode::GetUpvalue { index } => {
                    op_codes.push(op::GET_UPVALUE);
                    op_codes.push(*index);
                }
                Opcode::SetUpvalue { index } => {
                    op_codes.push(op::SET_UPVALUE);
                    op_codes.push(*index);
                }
                Opcode::GetGlobal { index } => {
                    op_codes.push(op::GET_GLOBAL);
                    op_codes.push(*index);
                }
                Opcode::SetGlobal { index } => {
                    op_codes.push(op::SET_GLOBAL);
                    op_codes.push(*index);
                }
                Opcode::DefineGlobal { index } => {
                    op_codes.push(op::DEFINE_GLOBAL);
                    op_codes.push(*index);
                }
                Opcode::GetField { index } => {
                    op_codes.push(op::GET_FIELD);
                    op_codes.push(*index);
                }
                Opcode::SetField { index } => {
                    op_codes.push(op::SET_FIELD);
                    op_codes.push(*index);
                }
                Opcode::BuildArray { size } => {
                    op_codes.push(op::BUILD_ARRAY);
                    op_codes.push(*size);
                }
                Opcode::GetArrayMethod { index } => {
                    op_codes.push(op::GET_ARRAY_METHOD);
                    op_codes.push(*index);
                }
                Opcode::ArrayElemAssign => op_codes.push(op::ARRAY_ELEM_ASSIGN),
                Opcode::BinaryGetelem => op_codes.push(op::BINARY_GETELEM),
                Opcode::Call { arg_count } => {
                    op_codes.push(op::CALL);
                    op_codes.push(*arg_count);
                }
                Opcode::Method { index } => {
                    op_codes.push(op::METHOD);
                    op_codes.push(*index);
                }
                Opcode::Struct { index } => {
                    op_codes.push(op::STRUCT);
                    op_codes.push(*index);
                }
                Opcode::Field { index } => {
                    op_codes.push(op::FIELD);
                    op_codes.push(*index);
                }
                Opcode::Invoke { arg_count } => {
                    op_codes.push(op::INVOKE);
                    op_codes.push(*arg_count);
                }
                Opcode::SuperInvoke { arg_count } => {
                    op_codes.push(op::SUPER_INVOKE);
                    op_codes.push(*arg_count);
                }
                Opcode::GetSuper { index } => {
                    op_codes.push(op::GET_SUPER);
                    op_codes.push(*index);
                }
                Opcode::Jump { from, to } => {
                    op_codes.push(op::JUMP);
                    op_codes.push(from.clone());
                    op_codes.push(to.clone());
                }
                Opcode::JumpIfFalse { from, to } => {
                    op_codes.push(op::JUMP_IF_FALSE);
                    op_codes.push(from.clone());
                    op_codes.push(to.clone());
                }
                Opcode::Loop { from, to } => {
                    op_codes.push(op::LOOP);
                    op_codes.push(from.clone());
                    op_codes.push(to.clone());
                }
            }
        }
        op_codes
    }
}
#[derive(Debug)]
pub struct Optimizer {
    pub chunk: Chunk,
    pub counted_chunk: CountedChunk,
}

impl Optimizer {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            counted_chunk: CountedChunk::new(chunk.constants.clone(), chunk.spans.clone()),
            chunk,
        }
    }

    pub fn iterate_opcodes(&mut self) {
        let mut idx = 0;
        while idx < self.chunk.op_codes.len() {
            idx = self.op(idx);
        }
    }
    pub fn op(&mut self, idx: usize) -> usize {
        match self.chunk.op_codes[idx] {
            op::CECILE_CONSTANT => {
                let constant_idx = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::Constant(constant_idx);
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::ADD => {
                let opcode = Opcode::Add;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::SUB => {
                let opcode = Opcode::Sub;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::MUL => {
                let opcode = Opcode::Mul;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::DIV => {
                let opcode = Opcode::Div;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::MODULO => {
                let opcode = Opcode::Modulo;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::NEG => {
                let opcode = Opcode::Neg;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::NOT => {
                let opcode = Opcode::Not;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::EQUAL => {
                let opcode = Opcode::Equal;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::NOT_EQUAL => {
                let opcode = Opcode::NotEqual;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::LESS_THAN => {
                let opcode = Opcode::LessThan;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::LESS_THAN_EQUAL => {
                let opcode = Opcode::LessThanEqual;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::GREATER_THAN => {
                let opcode = Opcode::GreaterThan;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::GREATER_THAN_EQUAL => {
                let opcode = Opcode::GreaterThanEqual;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::AND => {
                let opcode = Opcode::And;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::OR => {
                let opcode = Opcode::Or;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::TRUE => {
                let opcode = Opcode::True;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::FALSE => {
                let opcode = Opcode::False;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::NIL => {
                let opcode = Opcode::Nil;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::RETURN => {
                let opcode = Opcode::Return;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::POP => {
                let opcode = Opcode::Pop;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::PRINT => {
                let opcode = Opcode::Print;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::PRINT_LN => {
                let opcode = Opcode::PrintLn;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::CONCAT => {
                let opcode = Opcode::Concat;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::CLOSE_UPVALUE => {
                let opcode = Opcode::CloseUpvalue;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::INHERIT => {
                let opcode = Opcode::Inherit;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::GET_LOCAL => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::GetLocal { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::SET_LOCAL => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::SetLocal { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::GET_UPVALUE => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::GetUpvalue { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::SET_UPVALUE => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::SetUpvalue { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::GET_GLOBAL => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::GetGlobal { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::DEFINE_GLOBAL => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::DefineGlobal { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::SET_GLOBAL => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::SetGlobal { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::GET_FIELD => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::GetField { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::SET_FIELD => {
                let index = self.chunk.op_codes[idx + 1].clone();
                let opcode = Opcode::SetField { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::BUILD_ARRAY => {
                let size = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::BuildArray { size };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::GET_ARRAY_METHOD => {
                let index = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::GetArrayMethod { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::ARRAY_ELEM_ASSIGN => {
                let opcode = Opcode::ArrayElemAssign;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::BINARY_GETELEM => {
                let opcode = Opcode::BinaryGetelem;
                self.counted_chunk.op_codes.push(opcode);

                idx + 1
            }
            op::CALL => {
                let arg_count = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::Call { arg_count };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::METHOD => {
                let index = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::Method { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::STRUCT => {
                let index = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::Struct { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::FIELD => {
                let index = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::Field { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::INVOKE => {
                let arg_count = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::Invoke { arg_count };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::SUPER_INVOKE => {
                let arg_count = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::SuperInvoke { arg_count };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::GET_SUPER => {
                let index = self.chunk.op_codes[idx + 1];
                let opcode = Opcode::GetSuper { index };
                self.counted_chunk.op_codes.push(opcode);

                idx + 2
            }
            op::JUMP => {
                let from = self.chunk.op_codes[idx + 1];
                let to = self.chunk.op_codes[idx + 2];

                let opcode = Opcode::Jump { from, to };
                self.counted_chunk.op_codes.push(opcode);

                idx + 3
            }
            op::JUMP_IF_FALSE => {
                let from = self.chunk.op_codes[idx + 1];
                let to = self.chunk.op_codes[idx + 2];

                let opcode = Opcode::JumpIfFalse { from, to };
                self.counted_chunk.op_codes.push(opcode);

                idx + 3
            }
            op::LOOP => {
                let from = self.chunk.op_codes[idx + 1];
                let to = self.chunk.op_codes[idx + 2];

                let opcode = Opcode::Loop { from, to };
                self.counted_chunk.op_codes.push(opcode);

                idx + 3
            }

            op::CLOSURE => {
                let mut idx = idx + 1;
                let constant_idx = self.chunk.op_codes[idx].clone();
                let constant = &self.chunk.constants[constant_idx as usize];

                let function = unsafe { constant.as_object().function };
                let mut upvalues = Vec::new();

                for _ in 0..unsafe { (*function).upvalue_count } {
                    idx += 1;
                    let is_local = self.chunk.op_codes[idx];
                    let is_local_bool = if is_local == 0 { false } else { true };

                    idx += 1;
                    let upvalue_idx = self.chunk.op_codes[idx];

                    let upvalue = OptUpvalue::new(upvalue_idx, is_local_bool);
                    upvalues.push(upvalue);
                }

                let opcode = Opcode::Closure(Closure::new(function, upvalues));
                self.counted_chunk.op_codes.push(opcode);
                idx + 1
            }
            _byte => idx + 1,
        }
    }
}
