use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    let answer = Arc::new(Mutex::new(42));

    for thread_no in 0..5 {
        let changer = Arc::clone(&answer);
        thread::spawn(move || {
            let mut changer = changer.lock().unwrap();
            println!("Setting answer to thread_no: {}", thread_no + 1,);
            *changer = thread_no + 1;
        });
    }
    let ten_ms = Duration::from_millis(10);
    thread::sleep(ten_ms);

    if answer.is_poisoned() {
        println!("Mutex was poisoned :(");
    } else {
        println!("Mutex survived :)");
        let final_answer = answer.lock().unwrap();
        println!("Ended with answer: {}", final_answer);
    }
}
