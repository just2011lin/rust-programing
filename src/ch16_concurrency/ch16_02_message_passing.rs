use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    let ax = tx.clone();
    thread::spawn(move || {
        for i in 1..6 {
            ax.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for i in 6..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in rx {
        println!("Got: {i}");
    }
}
