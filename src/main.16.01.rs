use std::thread;
use std::time::Duration;

fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("新线程中的数字：{i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("主线程中的数字：{i}");
        thread::sleep(Duration::from_millis(1));
    }
}

fn fn_b() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        for i in v {
            print!("{i} ");
        }
        println!("");
    });
    handle.join().unwrap();
}
