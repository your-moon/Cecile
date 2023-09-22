use std::mem;

use crate::vm::object::{Object, StringObject};
use hashbrown::hash_map::RawEntryMut;
use hashbrown::HashMap;

#[derive(Debug)]
pub struct CeAllocation {
    pub strings: HashMap<String, *mut StringObject>,
    pub objects: Vec<Object>,
}

impl CeAllocation {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            objects: Vec::new(),
        }
    }
    pub fn alloc<T>(&mut self, object: impl CeAlloc<T>) -> T {
        object.alloc(self)
    }
}

impl Drop for CeAllocation {
    fn drop(&mut self) {
        println!("Dropping CeAllocation");
        for object in &self.objects {
            object.free();
        }
        for &string in self.strings.values() {
            unsafe { Box::from_raw(string) };
        }
    }
}

pub trait CeAlloc<T> {
    fn alloc(self, allocation: &mut CeAllocation) -> T;
}

impl<T> CeAlloc<*mut T> for T
where
    *mut T: Into<Object>,
{
    fn alloc(self, allocation: &mut CeAllocation) -> *mut T {
        let ptr = Box::into_raw(Box::new(self));
        let object = ptr.into();
        allocation.objects.push(object);
        ptr
    }
}

impl<S> CeAlloc<*mut StringObject> for S
where
    S: AsRef<str> + Into<String>,
{
    fn alloc(self, allocation: &mut CeAllocation) -> *mut StringObject {
        match allocation.strings.raw_entry_mut().from_key(self.as_ref()) {
            RawEntryMut::Occupied(entry) => *entry.get(),
            RawEntryMut::Vacant(entry) => {
                let string = self.into();
                let ptr = Box::into_raw(Box::new(StringObject::new(unsafe {
                    mem::transmute(string.as_str())
                })));
                entry.insert(string, ptr);
                ptr
            }
        }
    }
}
