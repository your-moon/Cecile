use std::fmt::{self, Debug, Display, Formatter};

use crate::cc_parser::ast::Type;

use super::{chunk::Chunk, value::Value};

#[derive(Clone, Copy, Eq)]
#[repr(C)]
pub union Object {
    pub string: *mut StringObject,
    pub main: *mut MainObject,
    pub function: *mut ObjectFunction,
    pub closure: *mut ClosureObject,
    pub native: *mut ObjectNative,
    pub upvalue: *mut UpvalueObject,
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
            ObjectType::Closure => {
                let _free = unsafe { Box::from_raw(self.closure) };
            }
            ObjectType::Upvalue => {
                let _free = unsafe { Box::from_raw(self.upvalue) };
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
            ObjectType::Function => write!(f, "<function {}>", unsafe {
                (*(*self.function).name).value
            }),
            ObjectType::Native => write!(f, "<native {}>", unsafe { (*self.native).native }),
            ObjectType::Closure => write!(f, "<closure {:?}>", unsafe {
                (*(*(*self.closure).function).name).value
            }),
            ObjectType::Upvalue => write!(f, "<upvalue {}>", unsafe { (*self.upvalue).value }),
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
impl_from_object!(closure, ClosureObject);
impl_from_object!(upvalue, UpvalueObject);

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
    Closure,
    Upvalue,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ObjectNative {
    pub common: MainObject,
    pub native: Native,
}

impl Into<Object> for &ObjectNative {
    fn into(self) -> Object {
        Object {
            native: Box::into_raw(Box::new(self.clone())),
        }
    }
}

impl ObjectNative {
    pub fn new(native: Native) -> Self {
        Self {
            common: MainObject {
                type_: ObjectType::Native,
                is_marked: false,
            },
            native,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Native {
    Clock,
    RandomNumber,
}

impl Display for Native {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Native::Clock => write!(f, "{}", "clock"),
            Native::RandomNumber => write!(f, "{}", "random_number"),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ObjectFunction {
    pub common: MainObject,
    pub name: *mut StringObject,
    pub arity_count: u8,
    pub upvalue_count: u16,
    pub chunk: Chunk,
    pub return_type: Option<Type>,
}

impl Into<Object> for &ObjectFunction {
    fn into(self) -> Object {
        Object {
            function: Box::into_raw(Box::new(self.clone())),
        }
    }
}

impl ObjectFunction {
    pub fn new(name: *mut StringObject, arity_count: u8, return_type: Option<Type>) -> Self {
        let common = MainObject {
            is_marked: false,
            type_: ObjectType::Function,
        };
        Self {
            common,
            name,
            arity_count,
            upvalue_count: 0,
            return_type,
            chunk: Chunk::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MainObject {
    pub type_: ObjectType,
    pub is_marked: bool,
}

#[derive(Debug, Clone, Copy)]
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
                is_marked: false,
            },
            value,
        }
    }
}

impl Into<Object> for StringObject {
    fn into(self) -> Object {
        Object {
            string: Box::into_raw(Box::new(self)),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ClosureObject {
    pub main: MainObject,
    pub function: *mut ObjectFunction,
    pub upvalues: Vec<*mut UpvalueObject>,
}

impl Into<Object> for &ClosureObject {
    fn into(self) -> Object {
        Object {
            closure: Box::into_raw(Box::new(self.clone())),
        }
    }
}

impl ClosureObject {
    pub fn new(function: *mut ObjectFunction, upvalues: Vec<*mut UpvalueObject>) -> Self {
        Self {
            main: MainObject {
                type_: ObjectType::Closure,
                is_marked: false,
            },
            function,
            upvalues,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UpvalueObject {
    pub common: MainObject,
    pub value: Value,
    pub closed: Value,
}

impl Into<Object> for &UpvalueObject {
    fn into(self) -> Object {
        Object {
            upvalue: Box::into_raw(Box::new(self.clone())),
        }
    }
}

impl UpvalueObject {
    pub fn new(value: Value) -> Self {
        Self {
            common: MainObject {
                type_: ObjectType::Upvalue,
                is_marked: false,
            },
            closed: Value::Nil,
            value,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_string_object_to_value() {
        let object: Object = StringObject::new("test").into();
        assert_eq!(object.type_(), ObjectType::String);
        assert_eq!(unsafe { (*object.string).value }, "test");
    }

    #[test]
    fn test_function_object_to_value() {
        let string_ptr: Object = StringObject::new("test").into();
        let object: Object = (&ObjectFunction::new(unsafe { string_ptr.string }, 123, None)).into();
        assert_eq!(object.type_(), ObjectType::Function);
        assert_eq!(unsafe { (*object.function).arity_count }, 123);
    }
}
