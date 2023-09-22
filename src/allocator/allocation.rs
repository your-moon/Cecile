use std::collections::HashMap;

pub struct Allocation {
    pub strings: HashMap<String, usize>,
    pub objects: Vec<usize>,
}

impl Allocation {
    pub fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> *mut T {
        object.alloc(self)
    }
}

pub trait CeAlloc<T> {
    fn alloc(self, allocation: &mut Allocation) -> *mut T;
}

impl<T> CeAlloc<T> for T {
    fn alloc(self, allocation: &mut Allocation) -> *mut T {
        let ptr = Box::into_raw(Box::new(self));
        allocation.objects.push(ptr as usize);
        ptr
    }
}
