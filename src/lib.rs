#![allow(dead_code)]

use string_traits::StringCalculate;
use wasm_bindgen::prelude::wasm_bindgen;
mod string_traits;
mod char_traits;
mod operation;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::string_traits::StringCalculate;

    #[test]
    fn parse_parentheses() {
        let expr = "((3*(1+1))+((2*2)*2))/(3-1)".to_string();
        println!("{}", expr.calculate())
    }
}

#[wasm_bindgen]
pub fn calculate(expression: String) -> i32 {
    expression.calculate()
}