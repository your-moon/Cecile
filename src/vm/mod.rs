use arrayvec::ArrayVec;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use rand::Rng;
use termcolor::StandardStream;

use crate::allocator::allocation::GcMark;
use crate::allocator::GLOBAL;
use crate::vm::value::Value;
use crate::{
    allocator::allocation::{CeAlloc, CeAllocation},
    vm::object::StringObject,
};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use std::{mem, ptr};

use self::compiler::{Compiler, Upvalue};
use self::error::{ErrorS, TypeError};
use self::object::{
    ClosureObject, Native, ObjectFunction, ObjectNative, ObjectType, UpvalueObject,
};
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
    next_gc: usize,

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
        globals.insert(clock_string, clock_native.into());
        globals.insert(random_number, random_number_native.into());
        VM {
            stack: Box::new([Value::default(); STACK_MAX]),
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
            next_gc: 1024 * 1024,
        }
    }

    pub fn run(&mut self, source: &str, stdout: &mut StandardStream) -> Result<(), Vec<ErrorS>> {
        let mut compiler = Compiler::new(self.allocator);
        let function = compiler.compile(source, self.allocator, stdout)?;
        self.run_function(function);
        Ok(())
    }

    pub fn run_function(&mut self, function: *mut ObjectFunction) {
        self.stack_top = self.stack.as_mut_ptr();

        self.frames.clear();
        self.frame = CallFrame {
            closure: self
                .allocator
                .alloc(ClosureObject::new(function, Vec::new())),
            ip: unsafe { (*function).chunk.code.as_ptr() },
            stack: self.stack_top,
        };

        loop {
            let function = unsafe { &mut *(*self.frame.closure).function };
            let idx = unsafe { self.frame.ip.offset_from((*function).chunk.code.as_ptr()) };
            (*function).chunk.disassemble_instruction(idx as usize);
            match self.read_u8() {
                op::SET_UPVALUE => self.set_upvalue(),
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
                // op::CLOSE_UPVALUE => self.close_upvalue(),
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
                print!("[ {} ]", unsafe { *stack_ptr });
                stack_ptr = unsafe { stack_ptr.add(1) };
            }
            println!();
        }
    }

    fn close_upvalue(&mut self, last: Value) {
        let mut idx = 0;
        while idx < self.open_upvalues.len() {
            let upvalue = unsafe { *self.open_upvalues.get_unchecked(idx) };
            if unsafe { (*upvalue).value } >= last {
                unsafe { (*upvalue).value = (*upvalue).value };
                self.open_upvalues.remove(idx);
            } else {
                idx += 1;
            }
        }
    }

    fn set_upvalue(&mut self) {
        let upvalue_idx = self.read_u8();
        let upvalue = unsafe {
            *(*self.frame.closure)
                .upvalues
                .get_unchecked(upvalue_idx as usize)
        };
        let value = self.peek(0);
        unsafe { (*upvalue).value = *value };
    }

    fn get_upvalue(&mut self) {
        let upvalue_idx = self.read_u8() as usize;
        let object = *unsafe { (*self.frame.closure).upvalues.get_unchecked(upvalue_idx) };
        let value = unsafe { (*object).value };
        self.push_to_stack(value);
    }

    fn closure(&mut self) {
        let function = unsafe { self.read_constant().as_object().function };

        let upvalue_count = unsafe { (*function).upvalue_count } as usize;
        let mut upvalues = Vec::with_capacity(upvalue_count);

        for _ in 0..upvalue_count {
            let is_local = self.read_u8();
            let upvalue_idx = self.read_u8() as usize;

            let upvalue = if is_local != 0 {
                let location = unsafe { *self.frame.stack.add(upvalue_idx) };
                self.capture_upvalue(location)
            } else {
                let upvalue_ =
                    unsafe { *(*self.frame.closure).upvalues.get_unchecked(upvalue_idx) };
                upvalue_
            };
            upvalues.push(upvalue);
        }

        let closure = self.alloc(ClosureObject::new(function, upvalues));
        self.push_to_stack(closure.into());
    }

    fn call(&mut self) {
        let arg_count = self.read_u8() as usize;
        let callee = self.peek(arg_count);
        self.call_value(unsafe { *callee }, arg_count as usize);
    }

    fn call_value(&mut self, callee: Value, arg_count: usize) {
        if callee.is_object() {
            let object = callee.as_object();
            match object.type_() {
                ObjectType::Closure => self.call_closure(unsafe { object.closure }, arg_count),
                // ObjectType::Native => self.call_native(object, arg_count),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }

    fn call_closure(&mut self, closure: *mut ClosureObject, arg_count: usize) {
        let function = unsafe { &mut *(*closure).function };
        if arg_count != unsafe { (*function).arity_count.into() } {
            todo!()
        }
        if self.frames.len() == FRAMES_MAX {
            todo!()
        }
        let frame = CallFrame {
            closure,
            ip: unsafe { (*function).chunk.code.as_ptr() },
            stack: self.peek(arg_count as usize),
        };
        unsafe {
            self.frames
                .push_unchecked(mem::replace(&mut self.frame, frame))
        };
    }

    fn capture_upvalue(&mut self, location: Value) -> *mut UpvalueObject {
        match self
            .open_upvalues
            .iter()
            .find(|&&upvalue| unsafe { (*upvalue).value } == location)
        {
            Some(&upvalue) => upvalue,
            None => {
                let upvalue = self.alloc(UpvalueObject::new(location));
                self.open_upvalues.push(upvalue);
                upvalue
            }
        }
    }

    fn greater(&mut self) {
        self.binary_op_number(|a, b| Value::from(a > b));
    }

    fn greater_equal(&mut self) {
        self.binary_op_number(|a, b| Value::from(a >= b));
    }

    fn less(&mut self) {
        self.binary_op_number(|a, b| Value::from(a < b));
    }

    fn less_equal(&mut self) {
        self.binary_op_number(|a, b| Value::from(a <= b));
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
        if !unsafe { *value }.to_bool() {
            self.frame.ip = unsafe { self.frame.ip.add(offset) };
        }
    }

    fn get_local(&mut self) {
        let stack_idx = self.read_u8() as usize;
        let local = unsafe { *self.frame.stack.add(stack_idx) };
        self.push_to_stack(local);
    }

    fn set_local(&mut self) {
        let stack_idx = self.read_u8() as usize;
        let local = unsafe { self.frame.stack.add(stack_idx) };
        let value = self.peek(0);
        unsafe { *local = *value };
    }

    fn set_global(&mut self) {
        let name = unsafe { self.read_constant().as_object().string };
        let value = unsafe { *self.peek(0) };
        match self.globals.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
            }
            Entry::Vacant(_) => todo!(),
        }
    }

    fn get_global(&mut self) {
        let name = unsafe { self.read_constant().as_object().string };
        let value = self.globals.get(&name).unwrap();
        self.push_to_stack(*value);
    }

    fn define_global(&mut self) {
        let name = unsafe { self.read_constant().as_object().string };
        let value = self.pop();
        self.globals.insert(name, value);
    }

    fn op_nil(&mut self) {
        self.push_to_stack(Value::NIL);
    }

    fn op_true(&mut self) {
        self.push_to_stack(Value::TRUE);
    }

    fn op_false(&mut self) {
        self.push_to_stack(Value::FALSE);
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
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            self.push_to_stack((a.as_number() % b.as_number()).into());
        } else {
            // Err(TypeError::UnsupportedOperandInfix {
            //     op: "%".to_string(),
            //     lt_type: a.type_().to_string(),
            //     rt_type: b.type_().to_string(),
            // })
            todo!()
        }
    }

    fn add(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            self.push_to_stack((a.as_number() + b.as_number()).into());
        }

        if a.is_object() && b.is_object() {
            let a = a.as_object();
            let b = b.as_object();

            if a.type_() == ObjectType::String && b.type_() == ObjectType::String {
                let result = unsafe { [(*a.string).value, (*b.string).value] }.concat();
                let result = Value::from(self.alloc(result));
                self.push_to_stack(result);
            }
        }

        // self.err(TypeError::UnsupportedOperandInfix {
        //     op: "+".to_string(),
        //     lt_type: a.type_().to_string(),
        //     rt_type: b.type_().to_string(),
        // })
    }

    fn sub(&mut self) {
        self.binary_op_number(|a, b| Value::from(a - b))
    }

    fn mul(&mut self) {
        self.binary_op_number(|a, b| Value::from(a * b))
    }

    fn div(&mut self) {
        self.binary_op_number(|a, b| Value::from(a / b))
    }

    fn binary_op_number(&mut self, op: fn(f64, f64) -> Value) {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            let value = op(a.as_number(), b.as_number());
            self.push_to_stack(value);
        } else {
            // Err(TypeError::UnsupportedOperandInfix {
            //     op: op_str.to_string(),
            //     lt_type: a.type_().to_string(),
            //     rt_type: b.type_().to_string(),
            // })
            todo!()
        }
    }

    fn equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack((rhs == lhs).into());
    }

    fn not_equal(&mut self) {
        let rhs = self.pop();
        let lhs = self.pop();

        self.push_to_stack((rhs != lhs).into());
    }

    fn negate(&mut self) {
        let value: value::Value = self.pop();
        self.push_to_stack(Value::from(-(value.as_number())));
    }

    fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> T {
        self.gc();
        let allc = self.allocator.alloc(object);
        allc
    }

    fn gc(&mut self) {
        println!("--- gc start");
        let mut stack_ptr = self.stack_top;
        while stack_ptr < self.stack.as_mut_ptr() {
            self.allocator.mark(unsafe { *stack_ptr });
            stack_ptr = unsafe { stack_ptr.add(1) };
        }

        for (&name, &value) in &self.globals {
            self.allocator.mark(name);
            self.allocator.mark(value);
        }
        //
        // //BUG
        let closure_name = unsafe { (*(*(*self.frame.closure).function).name).value };
        println!("INSIDE closure name: {:?}", closure_name);
        self.allocator.mark(self.frame.closure);

        //
        for frame in &self.frames {
            self.allocator.mark(frame.closure);
        }
        //
        for upvalue in &self.open_upvalues {
            self.allocator.mark(*upvalue);
        }
        //
        self.allocator.trace();
        self.allocator.sweep();
        //
        self.next_gc = GLOBAL.allocated_bytes() * 2;
        println!("--- gc end");
    }
}

#[derive(Debug)]
pub struct CallFrame {
    closure: *mut ClosureObject,
    ip: *const u8,
    stack: *mut Value,
}
