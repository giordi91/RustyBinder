use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_me(a: i32, b:i32)->i32
{
    println!("hello world from rust");
    a + b

}