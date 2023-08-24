use core::fmt;

pub const OPERATORS: [char; 4] = ['+', '-', '*', '/'];

pub struct Operation {
    pub(crate) left_side: Option<String>,
    pub(crate) right_side: Option<String>,
    pub(crate) operator: Option<char>
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_display: String;
        if self.left_side.is_some() {
            left_display = self.left_side.clone().unwrap();
        } else {
            left_display = "None".to_string();
        }

        let right_display: String;
        if self.right_side.is_some() {
            right_display = self.right_side.clone().unwrap();
        } else {
            right_display = "None".to_string();
        }

        let operator_display: String;
        if self.right_side.is_some() {
            operator_display = self.operator.clone().unwrap().to_string();
        } else {
            operator_display = "None".to_string();
        }

        write!(f, "< {} {}  {} >", left_display, operator_display, right_display)
    }
}