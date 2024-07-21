use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // fn_a();
    // fn_b();
    fn_c();
}

pub fn fn_a() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

pub fn fn_b() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{received} ");
    }
}

pub fn fn_c() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        for n in 1..5 {
            tx1.send(n).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for n in 5..12 {
            tx.send(n).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for n in rx {
        println!("n is {n}");
    }
}
