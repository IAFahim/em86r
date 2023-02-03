use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn hello(x: String) -> String{
    x
}

#[wasm_bindgen]
pub fn line_parser(line: String) -> String {
    let mut str = String::new();
    let original = "mov $a1 $a2";

    if line.starts_with("mov") {
        str.push_str("00");
        str.push_str(&original[3..]);
    } else {
        str.push_str(original);
    }
    str
}
