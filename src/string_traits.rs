use crate::operation::Operation;
use crate::operation::OPERATORS;

pub trait ToOperation {
    fn to_operation(self) -> Operation;
}

impl ToOperation for String {
    fn to_operation(self) -> Operation {
        let mut left_side: String = "".to_string();
        let mut right_side: String = "".to_string();
        let mut operator: char = '\0';

        for current_operator in OPERATORS {
            let sliced = self.split_once(current_operator);
            if let Some(x) = sliced {
                if x.0 != "" {left_side = x.0.to_string()}
                else {panic!("Empty side!")}
                if x.1 != "" {right_side = x.1.to_string()}
                else {panic!("Empty side!")}
                operator = current_operator;
                break;
            }
        }

        Operation {left_side, right_side, operator}
    }
}

pub trait HasOperators {
    fn has_operators(self) -> bool;
}

impl HasOperators for String {
    fn has_operators(self) -> bool {
        for current_char in self.chars() {
            if !current_char.is_numeric() {return true}
        }
        false
    }
}