#![allow(dead_code)]


mod string_traits;
mod char_traits;
mod operation;

pub mod pirox {
    use crate::string_traits::StringCalculate;
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen]
    pub fn calculate(expression: String) -> i32 {
        expression.calculate()
    }
}

#[cfg(test)]
mod tests {
    use crate::string_traits::StringCalculate;
    #[test]
    fn parse_parentheses() {
        let expr = "7/2*2".to_string().calculate();
        println!("{}", expr);
    }
}