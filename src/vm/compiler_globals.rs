use std::hash::BuildHasherDefault;

use hashbrown::HashMap;
use rustc_hash::FxHasher;

use crate::cc_parser::ast::Type;

#[derive(Debug, Clone)]
pub struct Globals {
    pub globals: HashMap<String, Type, BuildHasherDefault<FxHasher>>,
}

impl Globals {
    pub fn insert(&mut self, name: String, type_: Type) {
        self.globals.insert(name, type_);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.globals.get(name)
    }
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            globals: HashMap::default(),
        }
    }
}
