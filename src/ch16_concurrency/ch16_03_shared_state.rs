use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut m = counter.lock().unwrap();
            *m += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {:?}", counter.lock().unwrap());
}
