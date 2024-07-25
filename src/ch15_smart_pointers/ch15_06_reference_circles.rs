use std::cell::RefCell;
use std::fs;
use std::rc::{Rc, Weak};
use List::Con;

#[derive(Debug)]
enum List {
    Con(String, RefCell<Weak<List>>),
}

pub fn main() {
    let s = fs::read_to_string("test.txt");
    if let Ok(s) = s {
        for i in 1..1000 {
            let a = Rc::new(Con(s.clone(), RefCell::new(Weak::new())));
            let b = Rc::new(Con(s.clone(), RefCell::new(Rc::downgrade(&a))));
            let Con(s, ref_cell) = a.as_ref();
            *ref_cell.borrow_mut() = Rc::downgrade(&b);
            println!("i = {i}, s.len = {}", s.len());
        }
    }
}

// pub fn main() {
//     let a = Rc::new(10);
//     let mut s = Weak::new();
//     s = Rc::downgrade(&a);

//     println!("s = {s:?}");
// }
