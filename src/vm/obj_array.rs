use crate::cc_parser::ast::Type;

use super::{
    built_in::ArrayMethod,
    object::{MainObject, ObjectType, StringObject},
    value::Value,
};

#[derive(Debug)]
#[repr(C)]
pub struct ArrayObject {
    pub main: MainObject,
    pub values: Vec<Value>,
    pub value_type: Type,
}

impl ArrayObject {
    pub fn new(values: Vec<Value>, type_: Type) -> Self {
        Self {
            main: MainObject {
                type_: ObjectType::Array(type_.clone()),
                is_marked: false,
            },
            values,
            value_type: type_,
        }
    }

    pub fn get_method(&self, name: *mut StringObject) -> Option<ArrayMethod> {
        match unsafe { (*name).value } {
            "push" => Some(ArrayMethod::Push),
            "pop" => Some(ArrayMethod::Pop),
            "len" => Some(ArrayMethod::Len),
            "remove" => Some(ArrayMethod::Remove),
            "insert" => Some(ArrayMethod::Insert),
            "clear" => Some(ArrayMethod::Clear),
            "reverse" => Some(ArrayMethod::Reverse),
            "sort" => Some(ArrayMethod::Sort),
            "get" => Some(ArrayMethod::Get),
            "get_type" => Some(ArrayMethod::Type),
            "copy" => Some(ArrayMethod::Copy),
            "extend" => Some(ArrayMethod::Extend),
            _ => None,
        }
    }

    pub fn _get_copy(&self) -> Self {
        Self {
            main: MainObject {
                type_: ObjectType::Array(self.value_type.clone()),
                is_marked: false,
            },
            values: self.values.clone(),
            value_type: self.value_type.clone(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct BoundArrayMethodObject {
    pub common: MainObject,
    pub array: *mut ArrayObject,
    pub method: ArrayMethod,
}

impl BoundArrayMethodObject {
    pub fn new(array: *mut ArrayObject, method: ArrayMethod) -> Self {
        Self {
            common: MainObject {
                type_: ObjectType::BoundArrayMethod,
                is_marked: false,
            },
            array,
            method,
        }
    }
}
