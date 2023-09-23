use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy)]
#[repr(C)]
pub union Object {
    pub string: *mut StringObject,
    pub main: *mut MainObject,
}

impl Object {
    pub fn type_(&self) -> ObjectType {
        unsafe { (*self.main).type_ }
    }

    pub fn free(self) {
        match self.type_() {
            ObjectType::String => unsafe { Box::from_raw(self.string) },
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

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.main == other.main }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectType {
    String,
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
        write!(f, "{}", "string")
    }
}
