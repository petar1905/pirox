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
        let left_display: String;
        if self.left_side != "" {
            left_display = self.left_side.clone();
        } else {
            left_display = "None".to_string();
        }

        let right_display: String;
        if self.right_side != "" {
            right_display = self.right_side.clone();
        } else {
            right_display = "None".to_string();
        }

        let operator_display: String;
        if self.operator != '\0' {
            operator_display = self.operator.clone().to_string();
        } else {
            operator_display = "None".to_string();
        }

        write!(f, "Operation {} {} {} ", left_display, operator_display, right_display)
    }
}

impl Operation {
    pub fn calculate(&self) -> i32 {
        let left_result: i32;
        let right_result: i32;

        if self.left_side.clone().has_operators() {
            left_result = self.left_side.clone().to_operation().calculate()
        }
        else {left_result = self.left_side.parse().unwrap()}

        if self.right_side.clone().has_operators() {
            right_result = self.right_side.clone().to_operation().calculate()
        }
        else {right_result = self.right_side.parse().unwrap()}

        match self.operator {
            '+' => return left_result+right_result,
            '-' => return left_result-right_result,
            '*' => return left_result*right_result,
            '/' => return left_result/right_result,
            _ => return 0
        }
    }
}