use std::fmt::{self, Debug, Display, Formatter};

use crate::cc_parser::ast::Type;

use super::chunk::Chunk;

#[derive(Clone, Copy, Eq)]
#[repr(C)]
pub union Object {
    pub string: *mut StringObject,
    pub main: *mut MainObject,
    pub function: *mut ObjectFunction,
    pub native: *mut ObjectNative,
}

impl Object {
    pub fn type_(&self) -> ObjectType {
        unsafe { (*self.main).type_ }
    }

    pub fn free(self) {
        match self.type_() {
            ObjectType::Function => {
                let _free = unsafe { Box::from_raw(self.function) };
            }
            ObjectType::String => {
                let _free = unsafe { Box::from_raw(self.string) };
            }
            ObjectType::Native => {
                let _free = unsafe { Box::from_raw(self.native) };
            }
        };
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.type_() {
            ObjectType::String => write!(f, "{}", unsafe { (*self.string).value }),
            ObjectType::Function => write!(f, "{}", "function"),
            ObjectType::Native => write!(f, "<native {}>", unsafe { (*self.native).native }),
        }
    }
}

macro_rules! impl_from_object {
    ($name:tt, $type_:ty) => {
        impl From<*mut $type_> for Object {
            fn from($name: *mut $type_) -> Self {
                Self { $name }
            }
        }
    };
}

impl_from_object!(string, StringObject);
impl_from_object!(function, ObjectFunction);
impl_from_object!(native, ObjectNative);

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.main == other.main }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectType {
    String,
    Function,
    Native,
}

pub struct ObjectNative {
    pub common: MainObject,
    pub native: Native,
}

impl ObjectNative {
    pub fn new(native: Native) -> Self {
        Self {
            common: MainObject {
                type_: ObjectType::Native,
            },
            native,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Native {
    Clock,
}

impl Display for Native {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Native::Clock => write!(f, "{}", "clock"),
        }
    }
}

#[repr(C)]
pub struct ObjectFunction {
    pub common: MainObject,
    pub name: *mut StringObject,
    pub arity: u8,
    pub upvalue_count: u16,
    pub chunk: Chunk,
    pub return_type: Option<Type>,
}

impl ObjectFunction {
    pub fn new(name: *mut StringObject, arity: u8, return_type: Option<Type>) -> Self {
        let common = MainObject {
            type_: ObjectType::Function,
        };
        Self {
            common,
            name,
            arity,
            upvalue_count: 0,
            return_type,
            chunk: Chunk::default(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MainObject {
    pub type_: ObjectType,
}

#[repr(C)]
pub struct StringObject {
    pub main: MainObject,
    pub value: &'static str,
}

impl StringObject {
    pub fn new(value: &'static str) -> StringObject {
        StringObject {
            main: MainObject {
                type_: ObjectType::String,
            },
            value,
        }
    }
}

impl Debug for StringObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "String")
    }
}

impl Display for StringObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "String")
    }
}

impl Debug for ObjectFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Function")
    }
}

impl Display for ObjectFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Function")
    }
}
