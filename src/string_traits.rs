use string_builder::Builder;
use crate::char_traits::IsOperator;
use crate::operation::Operation;
pub trait ToOperation {
    fn to_operation(self) -> Operation;
    fn get_operator(self) -> (char, usize);
}
impl ToOperation for String {
    fn get_operator(self) -> (char, usize) {
        let str = self.as_str();
        let mut inside_par: bool = false;
        let mut layer: u32 = 0;

        for (idx, current_char) in str.chars().enumerate() {
            let start_par: bool = current_char == '(';
            let end_par: bool = current_char == ')';
            if start_par {
                inside_par = true;
                layer += 1;
            }
            if !inside_par && current_char.is_operator() {
                return (current_char, idx)
            }
            if end_par && layer > 0 {layer -= 1}
            if inside_par && layer == 0 {inside_par = false}
        }
        ('\0', usize::MAX)
    }

    fn to_operation(self) -> Operation {
        let operator: (char, usize) = self.clone().get_operator();
        let left_side: String = self.split_at(operator.1).0.to_string();
        let right_side: String = self.split_at(operator.1+1).1.to_string();
        //println!("{} {} {}", left_side, operator.0, right_side);
        Operation {left_side, right_side, operator: operator.0}
    }
}
pub trait HasOperators {
    fn has_operators(self) -> bool;
}
impl HasOperators for String {
    fn has_operators(self) -> bool {
        for current_char in self.chars() {
            if current_char.is_operator() {return true}
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
            if end_par && layer > 0 {layer -= 1}
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
        let mut expr_to_calc = binding.clone();
        expr_to_calc.pop();
        expr_to_calc.remove(0);

        expr_to_calc = if expr_to_calc.clone().has_parentheses() {
            expr_to_calc.remove_parentheses()
        } else {expr_to_calc};

        let conv = binding.as_str();
        let solution = expr_to_calc.to_operation().calculate();
        let parsed_expr = self.replace(conv, solution.to_string().as_str());

        if parsed_expr.clone().has_parentheses() {
            return parsed_expr.remove_parentheses()
        }
        parsed_expr
    }
}