use std::cell::RefCell;
fn main() {
    let answer = RefCell::new(0);
    *answer.borrow_mut() = 42;
    println!("The answer is : {}", answer.borrow());
}
