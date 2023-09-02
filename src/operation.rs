use core::fmt;
use crate::string_traits::{HasOperators, ToOperation};
pub const OPERATORS: [char; 4] = ['+', '-', '*', '/'];

pub struct Operation {
    pub(crate) left_side: String,
    pub(crate) right_side: String,
    pub(crate) operator: char
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let left_display: String = if self.left_side.is_empty() {
            self.left_side.clone()
        } else {"None".to_string()};
        
        let right_display: String = if self.right_side.is_empty() {
            self.right_side.clone()
        } else {"None".to_string()};
        
        let operator_display: String = if self.operator != '\0' {
            self.operator.clone().to_string()
        } else {"None".to_string()};

        write!(f, "Operation {} {} {} ", left_display, operator_display, right_display)
    }
}

impl Operation {
    pub fn calculate(&self) -> i32 {
        let left_result: i32 = if self.left_side.clone().has_operators() {
            self.left_side.clone().to_operation().calculate()
        } else {self.left_side.parse().unwrap()};

        let right_result: i32 = if self.right_side.clone().has_operators() {
            self.right_side.clone().to_operation().calculate()
        } else {self.right_side.parse().unwrap()};

        match self.operator {
            '+' => left_result+right_result,
            '-' => left_result-right_result,
            '*' => left_result*right_result,
            '/' => left_result/right_result,
            _ => 0
        }
    }
}