fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    let a = Box::new(5);
    println!("a = {a}");
}

#[derive(Debug)]
enum List {
    Con(i32, Box<List>),
    Nil,
}

use List::{Con, Nil};

fn fn_b() {
    let mut b = Con(1, Box::new(Con(2, Box::new(Nil))));
    loop {
        if let Con(i, l) = b {
            println!("b = ({i}, {l:?})");
            b = *l;
        } else {
            break;
        }
    }
}
