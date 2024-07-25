use std::thread;
use std::time::Duration;

pub fn main() {
    let mut v: Vec<i32> = vec![];

    let handle = thread::spawn(move || {
        for i in 1..5 {
            v.push(i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("v = {v:?}");
    });

    handle.join().unwrap();
}
