use std::hash::BuildHasherDefault;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::vm::value::Value;
use crate::{
    allocator::allocation::{CeAlloc, CeAllocation},
    vm::object::StringObject,
};
use rustc_hash::FxHasher;
use std::{mem, ptr};
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
            self.chunk.disassemble_instruction(self.ip);
            match self.read_byte() {
                op::GET_LOCAL => self.get_local(),
                op::SET_LOCAL => self.set_local(),
                op::SET_GLOBAL => self.set_global(),
                op::PRINT => {
                    let value: value::Value = self.pop();
                    println!("{}", value);
                }
                op::ADD => self.add(),
                op::SUB => self.sub(),
                op::MUL => self.mul(),
                op::DIV => self.div(),
                op::EQUAL => self.equal(),
                op::NOT_EQUAL => self.not_equal(),
                op::NEG => self.negate(),
                op::CECILE_CONSTANT => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                op::DEFINE_GLOBAL => self.define_global(),
                op::GET_GLOBAL => self.get_global(),
                op::TRUE => self.op_true(),
                op::FALSE => self.op_false(),
                op::NIL => self.op_nil(),
                op::POP => {
                    self.pop();
                }
                op::RETURN => {
                    self.pop();
                    return;
                }
                _ => todo!(),
            }
            //print top of stack element
            let mut stack_ptr = self.stack.as_mut_ptr();
            while stack_ptr < self.stack_top {
                print!("[ {} ]", unsafe { *stack_ptr });
                stack_ptr = unsafe { stack_ptr.add(1) };
            }
            println!();
        }
    }

    fn get_local(&mut self) {
        let stack_index = self.read_constant();
        match stack_index {
            Value::Number(stack_index) => {
                let value = self.stack[stack_index as usize];
                self.push(value);
            }
            _ => todo!(),
        }
    }

    fn set_local(&mut self) {
        let stack_index = self.read_constant();
        match stack_index {
            Value::Number(stack_index) => {
                let value = self.pop();
                self.stack[stack_index as usize] = value;
            }
            _ => todo!(),
        }
    }

    fn set_global(&mut self) {
        let value = self.pop();
        let name = self.read_constant();
        match name {
            Value::String(ptr_string) => match self.globals.entry(ptr_string) {
                Entry::Occupied(mut entry) => {
                    entry.insert(value);
                }
                Entry::Vacant(entry) => {
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
                    self.push(*value);
                }
                None => {
                    todo!()
                }
            },
            _ => todo!(),
        };
    }

    fn define_global(&mut self) {
        let value = self.pop();
        let name = self.read_constant();
        let name = match name {
            Value::String(ptr_string) => {
                self.globals.insert(ptr_string, value);
            }
            _ => todo!(),
        };
    }

    fn op_nil(&mut self) {
        self.push(value::Value::Number(0.0));
    }

    fn op_true(&mut self) {
        self.push(true.into());
    }

    fn op_false(&mut self) {
        self.push(false.into());
    }

    fn push(&mut self, value: Value) {
        unsafe {
            ptr::write(self.stack_top, value);
            self.stack_top = self.stack_top.add(1);
        }
    }

    fn pop(&mut self) -> Value {
        unsafe {
            self.stack_top = self.stack_top.sub(1);
            ptr::read(self.stack_top)
        }
    }

    fn peek(&self, distance: usize) -> *mut Value {
        unsafe { self.stack_top.sub(distance + 1) }
    }

    fn read_constant(&mut self) -> value::Value {
        let index = self.read_byte() as usize;
        let constant = self.chunk.constants[index].clone();
        constant
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn add(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        match (lhs, rhs) {
            (value::Value::Number(lhs), value::Value::Number(rhs)) => {
                // println!(" adding {} + {}", lhs, rhs);
                self.push(Value::Number(lhs + rhs));
            }
            (value::Value::String(lhs), value::Value::String(rhs)) => {
                let mut lhs = unsafe { (*lhs).value.clone() };
                let mut rhs = unsafe { (*rhs).value.clone() };
                let string = lhs.to_string() + &rhs.to_string();
                // let static_str: &'static str = Box::leak(string.into_boxed_str());
                // let string_obj = StringObject::new(static_str);
                let ptr_string = self.alloc(string);
                let string = Value::String(ptr_string);
                self.push(string);
            }
            _ => todo!(),
        }
    }

    fn sub(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push(lhs.sub(rhs));
    }

    fn mul(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push(lhs.mul(rhs));
    }

    fn div(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push(lhs.div(rhs));
    }

    fn equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push((lhs == rhs).into());
    }

    fn not_equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();

        self.push((lhs != rhs).into());
    }

    fn negate(&mut self) {
        let value: value::Value = self.pop();
        // Value to f64
        self.push(value.neg());
    }

    fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> T {
        self.allocator.alloc(object)
    }
}
