#![allow(dead_code)]

use string_traits::ToOperation;
use wasm_bindgen::prelude::wasm_bindgen;
mod string_traits;
mod operation;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::string_traits::ToOperation;
    #[test]
    fn it_works() {
        let expr = "9/3*7+5/5-6".to_string().to_operation().calculate();
        assert_eq!(expr, 16);

    }
}

#[wasm_bindgen]
pub fn calculate(expression: String) -> i32 {
    expression.to_operation().calculate()
}