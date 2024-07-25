use std::rc::Rc;

#[derive(Debug)]
enum List {
    Con(i32, Rc<List>),
    Nil,
}

use List::{Con, Nil};

pub fn main() {
    let a = Rc::new(Con(1, Rc::new(Nil)));
    let b = Con(2, Rc::clone(&a));
    let c = Con(3, Rc::clone(&a));

    if let Con(i, r) = &b {
        println!("{i}, {r:?}");
    }
    if let Con(i, r) = &c {
        println!("{i}, {r:?}");
    }
}
