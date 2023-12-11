use std::hash::BuildHasherDefault;

use hashbrown::HashMap;
use rustc_hash::FxHasher;

use crate::cc_parser::ast::Type;

#[derive(Debug, Clone)]
pub struct CompilerGlobals {
    pub globals: HashMap<String, Type, BuildHasherDefault<FxHasher>>,
}

impl CompilerGlobals {
    pub fn insert(&mut self, name: String, type_: Type) {
        self.globals.insert(name, type_);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.globals.get(name)
    }
}

impl Default for CompilerGlobals {
    fn default() -> Self {
        Self {
            globals: HashMap::default(),
        }
    }
}
