use std::rc::Rc;

fn main() {
    fn_a();
}

#[derive(Debug)]
enum List {
    Con(i32, Rc<List>),
    Nil,
}

use crate::List::{Con, Nil};

fn fn_a() {
    let a = Con(2, Rc::new(Con(1, Rc::new(Nil))));
    let aa = Con(3, Rc::new(a));
    let mut _a = &aa;
    loop {
        if let Con(i, l) = _a {
            println!("i = {i}");
            _a = l;
        } else {
            break;
        }
    }
    println!("aa = {aa:?}");
}
