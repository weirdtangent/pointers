use std::sync::Arc;
use std::thread;
use std::time::Duration;
fn main() {
    let answer = Arc::new(42);

    for threadno in 0..5 {
        let answer = Arc::clone(&answer);
        thread::spawn(move || {
            println!("Thread {}, answer is: {}", threadno + 1, answer);
        });
    }
    let ten_ms = Duration::from_millis(10);
    thread::sleep(ten_ms);
}
