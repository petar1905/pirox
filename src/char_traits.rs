pub trait IsOperator {
    fn is_operator(self) -> bool;
}

impl IsOperator for char {
    fn is_operator(self) -> bool {
        match self {
            '+' | '-' => true,
            '*' | '/' => true,
            _ => false
        }
    }
}