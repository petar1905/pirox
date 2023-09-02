#![allow(dead_code)]

use string_traits::ToOperation;
use wasm_bindgen::prelude::wasm_bindgen;
mod string_traits;
mod char_traits;
mod operation;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::string_traits::{ToOperation, RemoveParentheses};
    #[test]
    fn parse_parentheses() {
        let mut expr = "((3*(1+1))+((2*2)*2))/(3-1)".to_string().to_operation();
        expr.left_side = expr.left_side.remove_parentheses();
        expr.right_side = expr.right_side.remove_parentheses();
        println!("{}", expr)
    }
}

#[wasm_bindgen]
pub fn calculate(expression: String) -> i32 {
    expression.to_operation().calculate()
}