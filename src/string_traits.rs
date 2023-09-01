use string_builder::Builder;
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

pub trait RemoveParentheses {
    fn remove_parentheses(self) -> String;
    fn has_parentheses(self) -> bool;
    fn get_parentheses_pattern(self) -> String;
}

impl RemoveParentheses for String {
    fn get_parentheses_pattern(self) -> String {
        let str = self.as_str();
        let mut buffer: Builder = Builder::default();
        let mut inside_par: bool = false;
        let mut layer = 0;

        for current_char in str.chars() {
            let start_par: bool = current_char == '(';
            let end_par: bool = current_char == ')';
            if start_par {
                inside_par = true;
                layer += 1;
            }
            if inside_par {buffer.append(current_char)}
            if end_par {if layer > 0 {layer -= 1}}
            if inside_par && layer == 0 {break;}
        }
        buffer.string().unwrap()
    }

    fn has_parentheses(self) -> bool {
        for current_char in self.chars() {
            if current_char == '(' || current_char == ')' {return true;}
        }
        false
    }

    fn remove_parentheses(self) -> String {
        let binding = self.clone().get_parentheses_pattern();
        let mut expr_to_calc = binding.replace("(", "");
        expr_to_calc = expr_to_calc.replace(")", "");

        let conv = binding.as_str();
        let solution = expr_to_calc.to_operation().calculate();
        let parsed_expr = self.replace(conv, solution.to_string().as_str());

        if parsed_expr.clone().has_parentheses() {
            return parsed_expr.remove_parentheses()
        }
        parsed_expr
    }
}