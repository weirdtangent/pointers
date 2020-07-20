use std::rc::Rc;
fn main() {
    let correct_answer = Rc::new(42);
    let my_answer = Rc::clone(&correct_answer);

    println!("The correct answer is : {}", correct_answer);
    drop(correct_answer);

    println!("And you got : {}", my_answer);
}
