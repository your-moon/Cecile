use arrayvec::ArrayVec;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use rand::Rng;
use termcolor::StandardStream;

use crate::vm::value::Value;
use crate::{
    allocator::allocation::{CeAlloc, CeAllocation},
    vm::object::StringObject,
};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use std::{mem, ptr};

use self::compiler::Compiler;
use self::error::ErrorS;
use self::object::{ClosureObject, Native, ObjectFunction, ObjectNative, UpvalueObject};
pub mod chunk;
pub mod compiler;
pub mod error;
pub mod object;
pub mod op;
pub mod value;

const FRAMES_MAX: usize = 64;
const STACK_MAX: usize = FRAMES_MAX * STACK_MAX_PER_FRAME;
const STACK_MAX_PER_FRAME: usize = u8::MAX as usize + 1;

pub struct VM<'a> {
    stack: Box<[Value; STACK_MAX]>,
    frames: ArrayVec<CallFrame, FRAMES_MAX>,
    frame: CallFrame,
    stack_top: *mut Value,
    allocator: &'a mut CeAllocation,
    globals: HashMap<*mut StringObject, Value, BuildHasherDefault<FxHasher>>,
    open_upvalues: Vec<*mut UpvalueObject>,
}

impl<'a> VM<'a> {
    pub fn new(allocator: &'a mut CeAllocation) -> VM {
        let mut globals = HashMap::with_capacity_and_hasher(256, BuildHasherDefault::default());
        let clock_string = allocator.alloc("clock");
        let random_number = allocator.alloc("random_number");
        let clock_native = allocator.alloc(ObjectNative::new(Native::Clock));
        let random_number_native = allocator.alloc(ObjectNative::new(Native::RandomNumber));
        globals.insert(clock_string, Value::Native(clock_native));
        globals.insert(random_number, Value::Native(random_number_native));
        VM {
            stack: Box::new([Value::Number(0.0); STACK_MAX]),
            stack_top: ptr::null_mut(),
            allocator,
            globals,
            frames: ArrayVec::new(),
            frame: CallFrame {
                closure: ptr::null_mut(),
                ip: ptr::null_mut(),
                stack: ptr::null_mut(),
            },
            open_upvalues: Vec::new(),
        }
    }

    pub fn run(&mut self, source: &str, stdout: &mut StandardStream) -> Result<(), Vec<ErrorS>> {
        let mut compiler = Compiler::new(self.allocator);
        let function = compiler.compile(source, self.allocator, stdout)?;
        // println!("{:?}", self.allocator);
        // println!("{:?}", unsafe { (*function).chunk.disassemble("script") });
        self.run_function(function);
        Ok(())
    }

    pub fn run_function(&mut self, function: *mut ObjectFunction) {
        self.stack_top = self.stack.as_mut_ptr();

        self.frames.clear();
        self.frame = CallFrame {
            closure: self.alloc(ClosureObject::new(function, Vec::new())),
            ip: unsafe { (*function).chunk.code.as_ptr() },
            stack: self.stack_top,
        };

        loop {
            let function = unsafe { &mut *(*self.frame.closure).function };
            let idx = unsafe { self.frame.ip.offset_from((*function).chunk.code.as_ptr()) };
            (*function).chunk.disassemble_instruction(idx as usize);
            match self.read_u8() {
                op::GET_UPVALUE => self.get_upvalue(),
                op::CLOSURE => self.closure(),
                op::MODULO => self.modulo(),
                op::CALL => self.call(),
                op::LOOP => self.loop_(),
                op::JUMP => self.jump(),
                op::JUMP_IF_FALSE => self.jump_if_false(),
                op::GET_LOCAL => self.get_local(),
                op::SET_LOCAL => self.set_local(),
                op::SET_GLOBAL => self.set_global(),
                op::PRINT => {
                    let value: value::Value = self.pop();
                    print!("{}", value);
                }
                op::PRINT_LN => {
                    let value: value::Value = self.pop();
                    println!("{}", value);
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
                    self.pop();
                }
                op::RETURN => {
                    let value = self.pop();

                    self.stack_top = self.frame.stack;
                    match self.frames.pop() {
                        Some(frame) => {
                            self.frame = frame;
                        }
                        None => {
                            return;
                        }
                    }
                    self.push_to_stack(value);
                }
                _ => todo!(),
            }
            // print top of stack element
            print!("    ");
            let mut stack_ptr = self.frame.stack;
            while stack_ptr < self.stack_top {
                eprint!("[ {:?} ]", unsafe { *stack_ptr });
                stack_ptr = unsafe { stack_ptr.add(1) };
            }
            println!();
        }
    }

    fn get_upvalue(&mut self) {
        let upvalue_idx = self.read_u8();
        let upvalue = unsafe {
            *(*self.frame.closure)
                .upvalues
                .get_unchecked(upvalue_idx as usize)
        };
        let value = unsafe { (*upvalue).value };
        self.push_to_stack(value);
    }

    fn closure(&mut self) {
        let function = self.read_constant();
        let function = match function {
            Value::Function(ptr_function) => unsafe { &mut *ptr_function },
            _ => todo!(),
        };
        let mut upvalues = Vec::new();
        for _ in 0..(*function).upvalue_count {
            let is_local = self.read_u8();
            let index = self.read_u8() as usize;
            let upvalue = if is_local == 1 {
                let stack_ptr = unsafe { *self.frame.stack.add(index as usize) };
                self.capture_upvalue(stack_ptr)
            } else {
                let upvalue = unsafe { *(*self.frame.closure).upvalues.get_unchecked(index) };
                upvalue
            };
            upvalues.push(upvalue);
        }
        let closure = self.alloc(ClosureObject::new(function, upvalues));
        self.push_to_stack(Value::Closure(closure));
    }

    fn call(&mut self) {
        let arg_count = self.read_u8();

        let function = self.peek(arg_count as usize);
        match unsafe { *function } {
            Value::Closure(closure) => {
                let frame = CallFrame {
                    closure,
                    ip: unsafe { (*(*closure).function).chunk.code.as_ptr() },
                    stack: self.peek(arg_count as usize),
                };
                unsafe {
                    self.frames
                        .push_unchecked(mem::replace(&mut self.frame, frame))
                }
            }
            Value::Native(ptr_native) => {
                self.pop();
                let native = unsafe { &mut *ptr_native };
                let result = match native.native {
                    Native::Clock => {
                        let now = std::time::SystemTime::now();
                        let duration = now
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("Time went backwards");
                        let seconds = duration.as_secs() as f64;
                        let nanos = duration.subsec_nanos() as f64 / 1_000_000_000.0;
                        Value::Number(seconds + nanos)
                    }
                    Native::RandomNumber => {
                        let mut rng = rand::thread_rng();
                        let number: f64 = rng.gen();
                        Value::Number(number)
                    }
                };
                self.stack_top = unsafe { self.stack_top.sub(arg_count as usize) };
                self.push_to_stack(result);
            }
            _ => todo!(),
        }
    }

    fn capture_upvalue(&mut self, value: Value) -> *mut UpvalueObject {
        match self
            .open_upvalues
            .iter()
            .find(|&&upvalue| unsafe { (*upvalue).value } == value)
        {
            Some(&upvalue) => upvalue,
            None => {
                let upvalue = self.alloc(UpvalueObject::new(value));
                self.open_upvalues.push(upvalue);
                upvalue
            }
        }
    }

    fn greater(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.greater(rhs));
    }

    fn greater_equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.greater_equal(rhs));
    }

    fn less(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.less(rhs));
    }

    fn less_equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.less_equal(rhs));
    }

    fn loop_(&mut self) {
        let offset = self.read_u16() as usize;
        self.frame.ip = unsafe { self.frame.ip.sub(offset) };
    }

    fn jump(&mut self) {
        let offset = self.read_u16() as usize;
        self.frame.ip = unsafe { self.frame.ip.add(offset) };
    }

    fn jump_if_false(&mut self) {
        let offset = self.read_u16() as usize;
        let value = self.peek(0);
        match unsafe { *value } {
            Value::Bool(value) => {
                if !value {
                    self.frame.ip = unsafe { self.frame.ip.add(offset) };
                }
            }
            _ => todo!(),
        }
    }

    fn get_local(&mut self) {
        let stack_idx = self.read_constant();
        match stack_idx {
            Value::Number(stack_idx) => {
                let value = unsafe { *self.frame.stack.add(stack_idx as usize) };
                self.push_to_stack(value);
            }
            _ => todo!(),
        }
    }

    fn set_local(&mut self) {
        let stack_idx = self.read_constant();
        match stack_idx {
            Value::Number(stack_idx) => {
                let value = self.peek(0);
                unsafe { *self.frame.stack.add(stack_idx as usize) = *value };
            }
            _ => todo!(),
        }
    }

    fn set_global(&mut self) {
        let value = unsafe { *self.peek(0) };
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
            Value::String(ptr_string) => {
                let value = *self.globals.get(&ptr_string).unwrap();
                self.push_to_stack(value);
            }
            _ => todo!(),
        };
    }

    fn define_global(&mut self) {
        let name = self.read_constant();
        let value = self.pop();
        match name {
            Value::String(ptr_string) => {
                self.globals.insert(ptr_string, value);
            }
            Value::Function(ptr_func) => {
                let func_name = unsafe { (*ptr_func).name };
                self.globals.insert(func_name, name);
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
        unsafe { *self.stack_top = value };
        self.stack_top = unsafe { self.stack_top.add(1) };
    }

    fn pop(&mut self) -> Value {
        self.stack_top = unsafe { self.stack_top.sub(1) };
        unsafe { *self.stack_top }
    }

    fn peek(&self, distance: usize) -> *mut Value {
        unsafe { self.stack_top.sub(distance + 1) }
    }

    fn read_constant(&mut self) -> value::Value {
        let index = self.read_u8() as usize;
        let function = unsafe { &mut *(*self.frame.closure).function };
        *unsafe { (*function).chunk.constants.get_unchecked(index) }
    }

    fn read_u8(&mut self) -> u8 {
        let byte = unsafe { *self.frame.ip };
        self.frame.ip = unsafe { self.frame.ip.add(1) };
        byte
    }

    fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8();
        let byte2 = self.read_u8();
        (byte1 as u16) << 8 | (byte2 as u16)
    }

    fn modulo(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.modulo(rhs));
    }

    fn add(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
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
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.sub(rhs));
    }

    fn mul(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.mul(rhs));
    }

    fn div(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(lhs.div(rhs));
    }

    fn equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack(Value::Bool(rhs == lhs));
    }

    fn not_equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();

        self.push_to_stack(Value::Bool(rhs != lhs));
    }

    fn negate(&mut self) {
        let value: value::Value = self.pop();
        // Value to f64
        self.push_to_stack(value.neg());
    }

    fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> T {
        self.allocator.alloc(object)
    }
}

#[derive(Debug)]
pub struct CallFrame {
    closure: *mut ClosureObject,
    ip: *const u8,
    stack: *mut Value,
}
