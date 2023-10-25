use crate::allocator::allocation::CeAllocation;
use crate::vm::object::ObjectType;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{
    compiler::Upvalue,
    object::{ClosureObject, Object, ObjectFunction, ObjectNative, StringObject, UpvalueObject},
};

#[derive(PartialEq, Clone, Copy, PartialOrd)]
pub enum Value {
    Number(f64),
    String(*mut StringObject),
    Function(*mut ObjectFunction),
    Closure(*mut ClosureObject),
    Upvalue(*mut UpvalueObject),
    Native(*mut ObjectNative),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn as_function(&self) -> *mut ObjectFunction {
        match self {
            Value::Function(ptr) => *ptr,
            _ => todo!(),
        }
    }
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => todo!(),
        }
    }

    pub fn as_object(&self) -> *mut Object {
        match self {
            Value::String(ptr) => *ptr as *mut Object,
            Value::Function(ptr) => *ptr as *mut Object,
            Value::Closure(ptr) => *ptr as *mut Object,
            Value::Upvalue(ptr) => *ptr as *mut Object,
            Value::Native(ptr) => *ptr as *mut Object,
            _ => todo!(),
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::String(_) => true,
            Value::Function(_) => true,
            Value::Closure(_) => true,
            Value::Upvalue(_) => true,
            Value::Native(_) => true,
            _ => false,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(ptr) => unsafe { write!(f, "{}", (**ptr).value) },
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Function(ptr) => write!(f, "<function {:?}>", unsafe { (*(**ptr).name).value }),
            Value::Native(ptr) => write!(f, "<native {:?}>", unsafe { (**ptr).native }),
            Value::Closure(ptr) => write!(f, "<closure {:?}>", unsafe {
                (*(*(**ptr).function).name).value
            }),
            Value::Upvalue(ptr) => write!(f, "<upvalue {:?}>", unsafe { (**ptr).value }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(ptr) => write!(f, "{}", unsafe { (**ptr).value }),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Function(ptr) => {
                write!(f, "<function {:?}>", unsafe { (*(*(*ptr)).name).value })
            }
            Value::Native(ptr) => write!(f, "<native {:?}>", unsafe { (**ptr).native }),
            Value::Closure(ptr) => write!(f, "<closure {:?}>", unsafe {
                (*(*(*(*ptr)).function).name).value
            }),
            Value::Upvalue(ptr) => write!(f, "<upvalue {:?}>", unsafe { (**ptr).value }),
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => todo!(),
        }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<*mut StringObject> for Value {
    fn from(s: *mut StringObject) -> Self {
        Value::String(s)
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => todo!(),
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => todo!(),
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => todo!(),
        }
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => todo!(),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Value {
        match self {
            Value::Number(n) => Value::Number(-n),
            _ => todo!(),
        }
    }
}

impl Value {
    pub fn modulo(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => todo!(),
        }
    }

    pub fn greater(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a > &b),
            _ => todo!(),
        }
    }

    pub fn greater_equal(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a >= &b),
            _ => todo!(),
        }
    }

    pub fn less(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a < &b),
            _ => todo!(),
        }
    }

    pub fn less_equal(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a <= &b),
            _ => todo!(),
        }
    }
}
