extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// Import 'window.alert'
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
// Export a 'helloworld' function
#[wasm_bindgen]
pub fn helloworld(name: &str) {
    alert(&format!("Hello World : {}!", name));
}
