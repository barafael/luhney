use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn process_str(input: String) -> String {
    input.to_ascii_uppercase()
}

#[wasm_bindgen]
pub fn help_text() -> String {
    "Here's some help text to get started. This example program will just shout back anything you give it".to_string()
}
