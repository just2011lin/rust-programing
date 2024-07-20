use std::rc::Rc;

fn main() {
    fn_a();
    fn_b();
}

#[derive(Debug)]
enum List {
    Con(i32, Rc<List>),
    Nil,
}

use List::{Con, Nil};

fn fn_a() {
    let c = Con(12, Rc::new(Nil));
    let b = Con(10, Rc::new(c));
    let a = Con(8, Rc::new(b));
    if let Con(n, l) = &a {
        println!("n is {n}, l is {l:?}");
    }
    println!("a is {a:?}");
}

fn fn_b() {
    let c = Con(12, Rc::new(Nil));
    let b = Rc::new(Con(10, Rc::new(c)));
    println!("b count is {}", Rc::strong_count(&b));
}
