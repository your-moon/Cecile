use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::BuildHasherDefault,
};

use hashbrown::HashMap;
use rustc_hash::FxHasher;

use crate::cc_parser::ast::Type;
use std::mem;

use super::{chunk::Chunk, value::Value};

const _: () = assert!(mem::size_of::<Object>() == 4 || mem::size_of::<Object>() == 8);

#[derive(Clone, Copy, Eq)]
#[repr(C)]
pub union Object {
    pub string: *mut StringObject,
    pub main: *mut MainObject,
    pub function: *mut ObjectFunction,
    pub closure: *mut ClosureObject,
    pub native: *mut ObjectNative,
    pub upvalue: *mut UpvalueObject,
    pub cstruct: *mut StructObject,
    pub instance: *mut InstanceObject,
    pub bound_method: *mut BoundMethodObject,
    pub array: *mut super::obj_array::ArrayObject,
    pub bound_array_method: *mut super::obj_array::BoundArrayMethodObject,
}

impl Object {
    pub fn _is_iterable(&self) -> bool {
        matches!(self.type_(), ObjectType::Array(_))
    }

    pub fn type_(&self) -> ObjectType {
        unsafe { (*self.main).type_.clone() }
    }
    pub fn is_array(&self) -> bool {
        matches!(self.type_(), ObjectType::Array(_))
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
            ObjectType::Struct => {
                let _free = unsafe { Box::from_raw(self.cstruct) };
            }
            ObjectType::Instance => {
                let _free = unsafe { Box::from_raw(self.instance) };
            }
            ObjectType::BoundMethod => {
                let _free = unsafe { Box::from_raw(self.bound_method) };
            }
            ObjectType::Array(_) => {
                let _free = unsafe { Box::from_raw(self.array) };
            }
            ObjectType::BoundArrayMethod => {
                let _free = unsafe { Box::from_raw(self.bound_array_method) };
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
            ObjectType::Closure => {
                write!(f, "{}", unsafe { Object::from((*self.closure).function) })
            }
            ObjectType::Upvalue => write!(f, "<upvalue>"),
            ObjectType::Struct => {
                write!(f, "<struct {}>", unsafe { (*(*self.cstruct).name).value })
            }
            ObjectType::Instance => {
                write!(f, "<instance {:?}>", unsafe {
                    &(*(*(*self.instance).struct_).name).value
                })
            }
            ObjectType::BoundMethod => write!(f, "<bound method {}>", unsafe {
                (*(*(*(*self.bound_method).method).function).name).value
            }),
            ObjectType::Array(_) => {
                for (i, value) in unsafe { (*self.array).values.iter().enumerate() } {
                    if i == 0 {
                        write!(f, "[{}", value)?;
                    } else {
                        write!(f, ", {}", value)?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
            ObjectType::BoundArrayMethod => write!(f, "<bound array method {:?}>", unsafe {
                (*self.bound_array_method).method
            }),
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
impl_from_object!(cstruct, StructObject);
impl_from_object!(instance, InstanceObject);
impl_from_object!(bound_method, BoundMethodObject);
impl_from_object!(array, super::obj_array::ArrayObject);
impl_from_object!(bound_array_method, super::obj_array::BoundArrayMethodObject);

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.main == other.main }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectType {
    String,
    Function,
    Native,
    Closure,
    Upvalue,
    Struct,
    Instance,
    BoundMethod,
    BoundArrayMethod,
    Array(Type),
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "{}", "string"),
            Self::Function => write!(f, "{}", "function"),
            Self::Native => write!(f, "{}", "native"),
            Self::Closure => write!(f, "{}", "closure"),
            Self::Upvalue => write!(f, "{}", "upvalue"),
            Self::Struct => write!(f, "{}", "struct"),
            Self::Instance => write!(f, "{}", "instance"),
            Self::BoundMethod => write!(f, "{}", "bound_method"),
            Self::Array(type_) => {
                write!(f, "Vec<{:?}>", type_)
            }
            Self::BoundArrayMethod => write!(f, "{}", "bound_array_method"),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ObjectNative {
    pub common: MainObject,
    pub native: Native,
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

#[derive(Debug)]
#[repr(C)]
pub struct ObjectFunction {
    pub main: MainObject,
    pub name: *mut StringObject,
    pub arity: u8,
    pub upvalue_count: u16,
    pub chunk: Chunk,
    pub return_type: Option<Type>,
}

impl ObjectFunction {
    pub fn new(name: *mut StringObject, arity_count: u8, return_type: Option<Type>) -> Self {
        let common = MainObject {
            is_marked: false,
            type_: ObjectType::Function,
        };
        Self {
            main: common,
            name,
            arity: arity_count,
            upvalue_count: 0,
            return_type,
            chunk: Chunk::default(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MainObject {
    pub type_: ObjectType,
    pub is_marked: bool,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
#[repr(C)]
pub struct ClosureObject {
    pub main: MainObject,
    pub function: *mut ObjectFunction,
    pub upvalues: Vec<*mut UpvalueObject>,
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

#[derive(Debug)]
#[repr(C)]
pub struct StructObject {
    pub main: MainObject,
    pub name: *mut StringObject,
    pub fields: HashMap<*mut StringObject, Value, BuildHasherDefault<FxHasher>>,
    pub methods: HashMap<*mut StringObject, *mut ClosureObject, BuildHasherDefault<FxHasher>>,
}

impl StructObject {
    pub fn new(name: *mut StringObject) -> Self {
        Self {
            main: MainObject {
                type_: ObjectType::Struct,
                is_marked: false,
            },
            name,
            methods: HashMap::default(),
            fields: HashMap::default(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct InstanceObject {
    pub main: MainObject,
    pub struct_: *mut StructObject,
    pub fields: HashMap<*mut StringObject, Value, BuildHasherDefault<FxHasher>>,
}

impl InstanceObject {
    pub fn new(struct_: *mut StructObject) -> Self {
        Self {
            main: MainObject {
                type_: ObjectType::Instance,
                is_marked: false,
            },
            struct_,
            fields: unsafe { (*struct_).fields.clone() },
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BoundMethodObject {
    pub common: MainObject,
    pub receiver: *mut InstanceObject,
    pub method: *mut ClosureObject,
}

impl BoundMethodObject {
    pub fn new(receiver: *mut InstanceObject, method: *mut ClosureObject) -> Self {
        Self {
            common: MainObject {
                type_: ObjectType::BoundMethod,
                is_marked: false,
            },
            receiver,
            method,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct UpvalueObject {
    pub main: MainObject,
    pub location: *mut Value,
    pub closed: Value,
}

impl UpvalueObject {
    pub fn new(location: *mut Value) -> Self {
        let main = MainObject {
            type_: ObjectType::Upvalue,
            is_marked: false,
        };
        Self {
            main,
            location,
            closed: Value::default(),
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_string_object_to_value() {}

    #[test]
    fn test_function_object_to_value() {}
}
