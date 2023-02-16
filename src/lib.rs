mod chunk;
mod compiler;
mod convert;
mod debug;
mod interpreter;
mod opcode;
mod statements;
mod tokenize;
mod utils;
mod value;
mod vm;

use interpreter::interpret;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run(_str: &str) {
    interpret("吾有一言")
}
