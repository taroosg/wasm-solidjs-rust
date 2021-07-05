extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
  value + 1
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
  match n {
    0 | 1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
