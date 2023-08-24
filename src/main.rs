mod string_traits;
mod operation;
use string_traits::ToOperation;

fn main() {
    println!("{}","2+2/2*3".to_string().to_operation().calculate());
}