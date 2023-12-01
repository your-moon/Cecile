#[derive(Debug, Clone, Copy)]
pub enum ArrayMethod {
    Extend,
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
    Type,
    None_,
    Copy,
}

impl Default for ArrayMethod {
    fn default() -> Self {
        Self::None_
    }
}

impl ArrayMethod {
    pub fn _from_str(name: &str) -> Self {
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
            "get_type" => Self::Type,
            "copy" => Self::Copy,
            "extend" => Self::Extend,

            _ => panic!("Unknown array method: {}", name),
        }
    }

    // pub fn call(&self, array: &mut ArrayObject, args: Vec<Value>) -> Object {}
}

pub fn builtin_array_methods_contains(name: &str) -> Option<String> {
    let arr = vec![
        "len", "push", "pop", "peek", "insert", "remove", "clear", "reverse", "sort", "get_type",
        "copy", "extend",
    ];

    if arr.contains(&name) {
        Some(name.to_string())
    } else {
        None
    }
}
