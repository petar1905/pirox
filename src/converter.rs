use crate::operation::Operation;
use crate::operation::OPERATORS;

pub trait ToOperation {
    fn to_operation(self) -> Operation;
}

impl ToOperation for String {
    fn to_operation(self) -> Operation {
        let mut left_side: Option<String> = None;
        let mut right_side: Option<String> = None;
        let mut operator: Option<char> = None;

        for current_operator in OPERATORS {
            let sliced = self.split_once(current_operator);
            if let Some(x) = sliced {
                println!("{} {} {}", x.0, current_operator, x.1);
                if x.0 != "" {left_side = Some(x.0.to_string())}
                if x.1 != "" {right_side = Some(x.1.to_string())}
                operator = Some(current_operator);
                break;
            }
        }

        Operation {left_side, right_side, operator}
    }
}