use arrayvec::ArrayVec;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use termcolor::StandardStream;

use crate::allocator::GLOBAL;
use crate::vm::value::Value;
use crate::{
    allocator::allocation::{CeAlloc, CeAllocation},
    vm::object::StringObject,
};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use std::{mem, ptr};

use self::compiler::Compiler;
use self::error::{AttributeError, Error, ErrorS, Result, TypeError};
use self::object::{
    ClosureObject, InstanceObject, Native, ObjectFunction, ObjectNative, ObjectType, StructObject,
    UpvalueObject,
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

    struct_init_method: *mut StringObject,
}

impl<'a> VM<'a> {
    pub fn new(allocator: &'a mut CeAllocation) -> VM {
        let mut globals = HashMap::with_capacity_and_hasher(256, BuildHasherDefault::default());
        let clock_string = allocator.alloc("clock");
        let random_number = allocator.alloc("random_number");
        let clock_native = allocator.alloc(ObjectNative::new(Native::Clock));
        let random_number_native = allocator.alloc(ObjectNative::new(Native::RandomNumber));
        let struct_init_method = allocator.alloc("new");
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
            struct_init_method,
        }
    }

    pub fn run(&mut self, source: &str, stdout: &mut StandardStream) -> Result<(), Vec<ErrorS>> {
        let mut compiler = Compiler::new(self.allocator);
        let function = compiler.compile(source, self.allocator, stdout)?;
        self.run_function(function).map_err(|e| vec![e])?;
        Ok(())
    }

    pub fn run_function(&mut self, function: *mut ObjectFunction) -> Result<()> {
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
                op::SET_FIELD => self.set_field(),
                op::GET_FIELD => self.get_field(),
                op::FIELD => self.field(),
                op::STRUCT => self.cstruct(),
                op::METHOD => self.method(),
                op::CECILE_CONSTANT => self.cecile_constant(),
                op::ADD => self.binary_add(),
                op::CONCAT => self.concat(),
                op::SUB => self.sub(),
                op::MUL => self.mul(),
                op::DIV => self.div(),
                op::EQUAL => self.equal(),
                op::NOT_EQUAL => self.not_equal(),
                op::NEG => self.negate(),
                op::MODULO => self.modulo(),
                op::GREATER_THAN => self.greater(),
                op::GREATER_THAN_EQUAL => self.greater_equal(),
                op::PRINT => self.op_print(),
                op::PRINT_LN => self.op_print_ln(),
                op::CALL => self.call(),
                op::CLOSURE => self.closure(),
                op::LOOP => self.loop_(),
                op::JUMP => self.jump(),
                op::JUMP_IF_FALSE => self.jump_if_false(),
                op::GET_LOCAL => self.get_local(),
                op::SET_LOCAL => self.set_local(),
                op::SET_UPVALUE => self.set_upvalue(),
                op::GET_UPVALUE => self.get_upvalue(),
                op::SET_GLOBAL => self.set_global(),
                op::LESS_THAN => self.less(),
                op::LESS_THAN_EQUAL => self.less_equal(),
                op::GET_GLOBAL => self.get_global(),
                op::DEFINE_GLOBAL => self.define_global(),
                op::TRUE => self.op_true(),
                op::FALSE => self.op_false(),
                op::NIL => self.op_nil(),
                // op::CLOSE_UPVALUE => self.close_upvalue(),
                op::POP => self.op_pop(),

                op::RETURN => {
                    let value = self.pop();

                    self.stack_top = self.frame.stack;
                    match self.frames.pop() {
                        Some(frame) => self.frame = frame,
                        None => break,
                    }
                    self.push_to_stack(value);
                    Ok(())
                }
                _ => todo!(),
            }?;

            // print top of stack element
            print!("    ");
            let mut stack_ptr = self.frame.stack;
            while stack_ptr < self.stack_top {
                print!("[ {} ]", unsafe { *stack_ptr });
                stack_ptr = unsafe { stack_ptr.add(1) };
            }
            println!();
        }
        Ok(())
    }

    fn set_field(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let instance = {
            let value = self.pop();
            let object = value.as_object();

            if value.is_object() && object.type_() == ObjectType::Instance {
                unsafe { object.instance }
            } else {
                return self.err(AttributeError::NoSuchAttribute {
                    type_: value.type_().to_string(),
                    name: unsafe { (*name).value.to_string() },
                });
            }
        };
        let value = self.peek(0);
        unsafe { (*(*instance).struct_).fields.insert(name, *value) };
        Ok(())
    }

    fn get_field(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let instance = {
            let value = unsafe { *self.peek(0) };
            let object = value.as_object();

            if value.is_object() && object.type_() == ObjectType::Instance {
                unsafe { object.instance }
            } else {
                return self.err(AttributeError::NoSuchAttribute {
                    type_: value.type_().to_string(),
                    name: unsafe { (*name).value.to_string() },
                });
            }
        };

        let value = unsafe { (*(*instance).struct_).fields.get(&name) };
        match value {
            Some(&value) => {
                self.push_to_stack(value);
            }
            None => {
                let method = unsafe { (*(*instance).struct_).methods.get(&name) };
                match method {
                    Some(&method) => {
                        self.call_closure(method, 0);
                    }
                    None => {
                        return self.err(AttributeError::NoSuchAttribute {
                            type_: "instance".to_string(),
                            name: unsafe { (*name).value.to_string() },
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn field(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let cstruct = unsafe { (*self.peek(0)).as_object().cstruct };
        unsafe { (*cstruct).fields.insert(name, Value::NIL) };
        Ok(())
    }
    fn cstruct(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let cstruct = self.alloc(StructObject::new(name));
        self.push_to_stack(cstruct.into());
        Ok(())
    }

    fn method(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let method = unsafe { self.pop().as_object().closure };
        let cstruct = unsafe { (*self.peek(0)).as_object().cstruct };
        unsafe { (*cstruct).methods.insert(name, method) };
        Ok(())
    }

    fn op_print(&mut self) -> Result<()> {
        let value: value::Value = self.pop();
        print!("{}", value);
        Ok(())
    }

    fn op_print_ln(&mut self) -> Result<()> {
        let value: value::Value = self.pop();
        println!("{}", value);
        Ok(())
    }

    fn op_pop(&mut self) -> Result<()> {
        self.pop();
        Ok(())
    }

    fn cecile_constant(&mut self) -> Result<()> {
        let constant = self.read_constant();
        self.push_to_stack(constant);
        Ok(())
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

    fn set_upvalue(&mut self) -> Result<()> {
        let upvalue_idx = self.read_u8();
        let upvalue = unsafe {
            *(*self.frame.closure)
                .upvalues
                .get_unchecked(upvalue_idx as usize)
        };
        let value = self.peek(0);
        unsafe { (*upvalue).value = *value };
        Ok(())
    }

    fn get_upvalue(&mut self) -> Result<()> {
        let upvalue_idx = self.read_u8() as usize;
        let object = *unsafe { (*self.frame.closure).upvalues.get_unchecked(upvalue_idx) };
        let value = unsafe { (*object).value };
        self.push_to_stack(value);
        Ok(())
    }

    fn closure(&mut self) -> Result<()> {
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
        Ok(())
    }

    fn call(&mut self) -> Result<()> {
        let arg_count = self.read_u8() as usize;
        let callee = self.peek(arg_count);
        self.call_value(unsafe { *callee }, arg_count as usize);
        Ok(())
    }

    fn call_value(&mut self, callee: Value, arg_count: usize) {
        if callee.is_object() {
            let object = callee.as_object();
            match object.type_() {
                ObjectType::Closure => self.call_closure(unsafe { object.closure }, arg_count),
                ObjectType::Struct => self.call_struct(unsafe { object.cstruct }, arg_count),
                // ObjectType::Native => self.call_native(object, arg_count),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }

    fn call_struct(&mut self, cstruct: *mut StructObject, arg_count: usize) {
        let instance = self.alloc(InstanceObject::new(cstruct));
        unsafe { *self.peek(arg_count) = Value::from(instance) };

        match unsafe { (*cstruct).methods.get(&self.struct_init_method) } {
            Some(&method) => {
                self.call_closure(method, arg_count);
            }
            None => {
                if arg_count != 0 {
                    todo!()
                }
            }
        }
    }

    fn call_closure(&mut self, closure: *mut ClosureObject, arg_count: usize) {
        let function = unsafe { &mut *(*closure).function };
        if arg_count != (*function).arity_count.into() {
            todo!()
        }
        if self.frames.len() == FRAMES_MAX {
            todo!()
        }
        let frame = CallFrame {
            closure,
            ip: (*function).chunk.code.as_ptr(),
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

    fn greater(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a > b))
    }

    fn greater_equal(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a >= b))
    }

    fn less(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a < b))
    }

    fn less_equal(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a <= b))
    }

    fn loop_(&mut self) -> Result<()> {
        let offset = self.read_u16() as usize;
        self.frame.ip = unsafe { self.frame.ip.sub(offset) };
        Ok(())
    }

    fn jump(&mut self) -> Result<()> {
        let offset = self.read_u16() as usize;
        self.frame.ip = unsafe { self.frame.ip.add(offset) };
        Ok(())
    }

    fn jump_if_false(&mut self) -> Result<()> {
        let offset = self.read_u16() as usize;
        let value = self.peek(0);
        if !unsafe { *value }.to_bool() {
            self.frame.ip = unsafe { self.frame.ip.add(offset) };
        }
        Ok(())
    }

    fn get_local(&mut self) -> Result<()> {
        let stack_idx = self.read_u8() as usize;
        let local = unsafe { *self.frame.stack.add(stack_idx) };
        self.push_to_stack(local);
        Ok(())
    }

    fn set_local(&mut self) -> Result<()> {
        let stack_idx = self.read_u8() as usize;
        let local = unsafe { self.frame.stack.add(stack_idx) };
        let value = self.peek(0);
        unsafe { *local = *value };
        Ok(())
    }

    fn set_global(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let value = unsafe { *self.peek(0) };
        match self.globals.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
            }
            Entry::Vacant(_) => todo!(),
        }
        Ok(())
    }

    fn get_global(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let value = self.globals.get(&name).unwrap();
        self.push_to_stack(*value);
        Ok(())
    }

    fn define_global(&mut self) -> Result<()> {
        let name = unsafe { self.read_constant().as_object().string };
        let value = self.pop();
        self.globals.insert(name, value);
        Ok(())
    }

    fn op_nil(&mut self) -> Result<()> {
        self.push_to_stack(Value::NIL);
        Ok(())
    }

    fn op_true(&mut self) -> Result<()> {
        self.push_to_stack(Value::TRUE);
        Ok(())
    }

    fn op_false(&mut self) -> Result<()> {
        self.push_to_stack(Value::FALSE);
        Ok(())
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

    fn modulo(&mut self) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            self.push_to_stack((a.as_number() % b.as_number()).into());
            return Ok(());
        }
        self.err(TypeError::UnsupportedOperandInfix {
            op: "%".to_string(),
            lt_type: a.type_().to_string(),
            rt_type: b.type_().to_string(),
        })
    }

    fn concat(&mut self) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        let a = a.as_object();
        let b = b.as_object();

        if a.type_() == ObjectType::String && b.type_() == ObjectType::String {
            let result = unsafe { [(*a.string).value, (*b.string).value] }.concat();
            let result = Value::from(self.alloc(result));
            self.push_to_stack(result);
            return Ok(());
        }

        self.err(TypeError::UnsupportedOperandInfix {
            op: "+".to_string(),
            lt_type: a.type_().to_string(),
            rt_type: b.type_().to_string(),
        })
    }

    fn binary_add(&mut self) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            self.push_to_stack((a.as_number() + b.as_number()).into());
            return Ok(());
        }

        self.err(TypeError::UnsupportedOperandInfix {
            op: "+".to_string(),
            lt_type: a.type_().to_string(),
            rt_type: b.type_().to_string(),
        })
    }

    fn sub(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a - b))
    }

    fn mul(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a * b))
    }

    fn div(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a / b))
    }

    fn binary_op_number(&mut self, op: fn(f64, f64) -> Value) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            let value = op(a.as_number(), b.as_number());
            self.push_to_stack(value);
            return Ok(());
        }
        self.err(TypeError::UnsupportedOperandInfix {
            op: "+".to_string(),
            lt_type: a.type_().to_string(),
            rt_type: b.type_().to_string(),
        })
    }

    fn equal(&mut self) -> Result<()> {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push_to_stack((rhs == lhs).into());
        Ok(())
    }

    fn not_equal(&mut self) -> Result<()> {
        let rhs = self.pop();
        let lhs = self.pop();

        self.push_to_stack((rhs != lhs).into());
        Ok(())
    }

    fn negate(&mut self) -> Result<()> {
        let value: value::Value = self.pop();
        self.push_to_stack(Value::from(-(value.as_number())));
        Ok(())
    }

    fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> T {
        if GLOBAL.allocated_bytes() > self.next_gc {
            self.gc();
        }
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

        self.allocator.mark(self.frame.closure);

        for frame in &self.frames {
            self.allocator.mark(frame.closure);
        }

        for upvalue in &self.open_upvalues {
            self.allocator.mark(*upvalue);
        }

        self.allocator.trace();
        self.allocator.sweep();

        self.next_gc = GLOBAL.allocated_bytes() * 2;

        println!("--- gc end");
    }

    fn err(&self, err: impl Into<Error>) -> Result<()> {
        let function = unsafe { (*self.frame.closure).function };
        let idx = unsafe { self.frame.ip.offset_from((*function).chunk.code.as_ptr()) } as usize;
        let span = unsafe { (*function).chunk.spans[idx - 1].clone() };
        Err((err.into(), span))
    }
}

#[derive(Debug)]
pub struct CallFrame {
    closure: *mut ClosureObject,
    ip: *const u8,
    stack: *mut Value,
}
