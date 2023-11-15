use super::{error::Result, object::ArrayObject, value::Value};

#[derive(Debug, Clone, Copy)]
pub enum ArrayMethod {
    Len,
    Push,
    Pop,
    Peek,
    Insert,
    Remove,
    Clear,
    Reverse,
    Sort,
    Get,
    None_,
}

impl Default for ArrayMethod {
    fn default() -> Self {
        Self::None_
    }
}

impl ArrayMethod {
    pub fn from_str(name: &str) -> Self {
        match name {
            "len" => Self::Len,
            "push" => Self::Push,
            "pop" => Self::Pop,
            "peek" => Self::Peek,
            "insert" => Self::Insert,
            "remove" => Self::Remove,
            "clear" => Self::Clear,
            "reverse" => Self::Reverse,
            "sort" => Self::Sort,
            "get" => Self::Get,
            _ => panic!("Unknown array method: {}", name),
        }
    }

    // pub fn call(&self, array: &mut ArrayObject, args: Vec<Value>) -> Object {}
}

pub fn builtin_array_methods_contains(name: &str) -> bool {
    let arr = vec![
        "len", "push", "pop", "peek", "insert", "remove", "clear", "reverse", "sort", "get",
    ];

    arr.contains(&name)
}
