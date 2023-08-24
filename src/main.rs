mod converter;
mod operation;
use converter::ToOperation;

fn main() {
    println!("{}","2*2-2".to_string().to_operation());
}