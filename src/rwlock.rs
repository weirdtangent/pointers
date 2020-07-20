use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
fn main() {
    let answer = Arc::new(RwLock::new(42));

    for thread_no in 0..5 {
        if thread_no % 2 == 1 {
            let changer = Arc::clone(&answer);
            thread::spawn(move || {
                let mut changer = changer.write().unwrap();
                println!("Setting answer to thread_no: {}", thread_no + 1,);
                *changer = thread_no + 1;
            });
        } else {
            let reader = Arc::clone(&answer);
            thread::spawn(move || {
                let reader = reader.read().unwrap();
                println!(
                    "Checking  answer in thread_no: {}, value is {}",
                    thread_no + 1,
                    *reader
                );
            });
        }
    }
    let ten_ms = Duration::from_millis(10);
    thread::sleep(ten_ms);

    if answer.is_poisoned() {
        println!("Mutex was poisoned :(");
    } else {
        println!("Mutex survived :)");
        let final_answer = answer.read().unwrap();
        println!("Ended with answer: {}", final_answer);
    }
}
