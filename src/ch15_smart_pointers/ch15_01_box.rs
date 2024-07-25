enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn collect(&self) -> Vec<i32> {
        let mut v = Vec::new();
        let mut b = self;
        while let Cons(i, second) = b {
            v.push(*i);
            b = second.as_ref();
        }
        v
    }
}

pub fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let vals = list.collect();
    println!("vals = {vals:?}");
}
