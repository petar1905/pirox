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
    use crate::string_traits::RemoveParentheses;
    #[test]
    fn parse_parentheses() {
        let expr = "((3*(1+1))+((2*2)*2))/(3-1)".to_string().get_parentheses_pattern();
        println!("{}", expr)
    }
}

#[wasm_bindgen]
pub fn calculate(expression: String) -> i32 {
    expression.to_operation().calculate()
}