use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
  value + 1
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
