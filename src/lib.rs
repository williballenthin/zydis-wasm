use zydis;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("push: {}", zydis::enums::Mnemonic::PUSH.get_string().unwrap()));
}

#[wasm_bindgen]
pub fn greet2(name: &str) {
    alert(&format!("Goodbye, {}!", name));
}
