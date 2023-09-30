use std::hash::BuildHasherDefault;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::vm::value::Value;
use crate::{
    allocator::allocation::{CeAlloc, CeAllocation},
    vm::object::StringObject,
};
use rustc_hash::FxHasher;
use std::ptr;
pub mod chunk;
pub mod compiler;
pub mod object;
pub mod op;
pub mod value;

#[derive(Debug)]
pub struct VM<'a> {
    chunk: chunk::Chunk,
    ip: usize,
    stack: Box<[Value; 256]>,
    stack_top: *mut Value,
    allocator: &'a mut CeAllocation,
    globals: HashMap<*mut StringObject, Value, BuildHasherDefault<FxHasher>>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: chunk::Chunk, allocator: &'a mut CeAllocation) -> VM {
        VM {
            chunk,
            ip: 0,
            stack: Box::new([Value::Number(0.0); 256]),
            stack_top: ptr::null_mut(),
            allocator,
            globals: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
        }
    }

    pub fn run(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
        loop {
            // self.chunk.disassemble_instruction(self.ip);
            match self.read_u8() {
                op::LOOP => self.loop_(),
                op::JUMP => self.jump(),
                op::JUMP_IF_FALSE => self.jump_if_false(),
                op::GET_LOCAL => self.get_local(),
                op::SET_LOCAL => self.set_local(),
                op::SET_GLOBAL => self.set_global(),
                op::PRINT => {
                    let value: value::Value = self.pop_from_stack();
                    print!("{}", value);
                }
                op::GREATER_THAN => self.greater(),
                op::GREATER_THAN_EQUAL => self.greater_equal(),
                op::LESS_THAN => self.less(),
                op::LESS_THAN_EQUAL => self.less_equal(),
                op::ADD => self.add(),
                op::SUB => self.sub(),
                op::MUL => self.mul(),
                op::DIV => self.div(),
                op::EQUAL => self.equal(),
                op::NOT_EQUAL => self.not_equal(),
                op::NEG => self.negate(),
                op::CECILE_CONSTANT => {
                    let constant = self.read_constant();
                    self.push_to_stack(constant);
                }
                op::DEFINE_GLOBAL => self.define_global(),
                op::GET_GLOBAL => self.get_global(),
                op::TRUE => self.op_true(),
                op::FALSE => self.op_false(),
                op::NIL => self.op_nil(),
                op::POP => {
                    self.pop_from_stack();
                }
                op::RETURN => {
                    self.pop_from_stack();
                    return;
                }
                _ => todo!(),
            }
            //print top of stack element
            // let mut stack_ptr = self.stack.as_mut_ptr();
            // while stack_ptr < self.stack_top {
            //     print!("[ {} ]", unsafe { *stack_ptr });
            //     stack_ptr = unsafe { stack_ptr.add(1) };
            // }
            // println!();
        }
    }

    fn greater(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.greater(rhs));
    }

    fn greater_equal(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.greater_equal(rhs));
    }

    fn less(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.less(rhs));
    }

    fn less_equal(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.less_equal(rhs));
    }

    fn loop_(&mut self) {
        let offset = self.read_u16() as usize;
        self.ip -= offset;
    }

    fn jump(&mut self) {
        let offset = self.read_u16() as usize;
        self.ip += offset;
    }

    fn jump_if_false(&mut self) {
        let offset = self.read_u16() as usize;
        let value = self.peek(0);
        match unsafe { *value } {
            Value::Bool(value) => {
                if !value {
                    self.ip += offset;
                }
            }
            _ => todo!(),
        }
    }

    fn get_local(&mut self) {
        let stack_idx = self.read_constant();
        match stack_idx {
            Value::Number(stack_idx) => {
                let value = self.stack[stack_idx as usize];
                self.push_to_stack(value);
            }
            _ => todo!(),
        }
    }

    fn set_local(&mut self) {
        let stack_index = self.read_constant();
        match stack_index {
            Value::Number(stack_index) => {
                let value = self.pop_from_stack();
                self.stack[stack_index as usize] = value;
            }
            _ => todo!(),
        }
    }

    fn set_global(&mut self) {
        let value = self.pop_from_stack();
        let name = self.read_constant();
        match name {
            Value::String(ptr_string) => match self.globals.entry(ptr_string) {
                Entry::Occupied(mut entry) => {
                    entry.insert(value);
                }
                Entry::Vacant(_entry) => {
                    todo!();
                }
            },
            _ => todo!(),
        };
    }

    fn get_global(&mut self) {
        let name = self.read_constant();
        match name {
            Value::String(ptr_string) => match self.globals.get(&ptr_string) {
                Some(value) => {
                    self.push_to_stack(*value);
                }
                None => {
                    todo!()
                }
            },
            _ => todo!(),
        };
    }

    fn define_global(&mut self) {
        let value = self.pop_from_stack();
        let name = self.read_constant();
        match name {
            Value::String(ptr_string) => {
                self.globals.insert(ptr_string, value);
            }
            _ => todo!(),
        };
    }

    fn op_nil(&mut self) {
        self.push_to_stack(value::Value::Nil);
    }

    fn op_true(&mut self) {
        self.push_to_stack(Value::Bool(true));
    }

    fn op_false(&mut self) {
        self.push_to_stack(Value::Bool(false));
    }

    fn push_to_stack(&mut self, value: Value) {
        unsafe {
            ptr::write(self.stack_top, value);
            self.stack_top = self.stack_top.add(1);
        }
    }

    fn pop_from_stack(&mut self) -> Value {
        unsafe {
            self.stack_top = self.stack_top.sub(1);
            ptr::read(self.stack_top)
        }
    }

    fn peek(&self, distance: usize) -> *mut Value {
        unsafe { self.stack_top.sub(distance + 1) }
    }

    fn read_constant(&mut self) -> value::Value {
        let index = self.read_u8() as usize;
        let constant = self.chunk.constants[index].clone();
        constant
    }

    fn read_u8(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8();
        let byte2 = self.read_u8();
        (byte1 as u16) << 8 | byte2 as u16
    }

    fn add(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        match (lhs, rhs) {
            (value::Value::Number(lhs), value::Value::Number(rhs)) => {
                // println!(" adding {} + {}", lhs, rhs);
                self.push_to_stack(Value::Number(lhs + rhs));
            }
            (value::Value::String(lhs), value::Value::String(rhs)) => {
                let lhs = unsafe { (*lhs).value.clone() };
                let rhs = unsafe { (*rhs).value.clone() };
                let string = lhs.to_string() + &rhs.to_string();
                // let static_str: &'static str = Box::leak(string.into_boxed_str());
                // let string_obj = StringObject::new(static_str);
                let ptr_string = self.alloc(string);
                let string = Value::String(ptr_string);
                self.push_to_stack(string);
            }
            _ => todo!(),
        }
    }

    fn sub(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.sub(rhs));
    }

    fn mul(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.mul(rhs));
    }

    fn div(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(lhs.div(rhs));
    }

    fn equal(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();
        self.push_to_stack(Value::Bool(rhs == lhs));
    }

    fn not_equal(&mut self) {
        let rhs = self.pop_from_stack();
        let lhs = self.pop_from_stack();

        self.push_to_stack(Value::Bool(rhs != lhs));
    }

    fn negate(&mut self) {
        let value: value::Value = self.pop_from_stack();
        // Value to f64
        self.push_to_stack(value.neg());
    }

    fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> T {
        self.allocator.alloc(object)
    }
}
