use core::fmt;
use std::{fmt::{Formatter, Display }, io};

use Cecile::vm::compiler::Compiler;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use std::io::Write;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hi this is a cecile wasm module!");
}
#[wasm_bindgen]
pub fn run_cecile() -> String {
    let mut output = &mut Output::new();
                let mut allocation = Cecile::allocator::allocation::CeAllocation::new();
                let mut compiler = Compiler::new(&mut allocation, false);
                let mut vm = Cecile::vm::VM::new(&mut allocation, false, false);
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum Message {
    ExitFailure,
    ExitSuccess,
    Output { text: String },
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).expect("could not serialize message"))
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = self)]
    fn postMessage(s: &str);
}



#[derive(Debug)]
struct Output;

impl Output {
    fn new() -> Self {
        Self
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let text = String::from_utf8_lossy(buf).to_string();
        postMessage(&Message::Output { text }.to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


