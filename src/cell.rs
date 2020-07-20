use std::cell::Cell;
fn main() {
    let answer = Cell::new(0);
    answer.set(42);
    println!("The answer is : {}", answer.get());
}
