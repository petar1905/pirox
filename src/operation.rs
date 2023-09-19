use core::fmt;
use crate::string_traits::{HasOperators, ToOperation};

pub const OPERATORS: [char; 5] = ['+', '-', '*', '/', '^']; 
pub struct Operation {
    pub(crate) left_side: String,
    pub(crate) right_side: String,
    pub(crate) operator: char
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation {} {} {} ", self.left_side, self.operator, self.right_side)
    }
}

impl Operation {
    pub fn calculate_f64(&self) -> f64 {
        let left_result: f64 = if self.left_side.clone().has_operators() {
            self.left_side.clone().to_operation().calculate_f64()
        } else {self.left_side.parse().unwrap()};
        
        let right_result: f64 = if self.right_side.clone().has_operators() {
            self.right_side.clone().to_operation().calculate_f64()
        } else {self.right_side.parse().unwrap()};

        match self.operator {
            '+' => (left_result+right_result).into(),
            '-' => (left_result-right_result).into(),
            '*' => (left_result*right_result).into(),
            '/' => (left_result/right_result).into(),
            '^' => (left_result.powf(right_result)).into(),
            _ => 0.0
        }
    }
}