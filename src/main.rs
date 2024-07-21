use std::rc::{Rc, Weak};
use std::{borrow::Borrow, cell::RefCell};

fn main() {
    fn_a();
    fn_b();
    fn_c();
}

fn fn_a() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));
    if let Cons(i, rc) = a.borrow() {
        println!("i = {i}");
        *rc.borrow_mut() = Rc::clone(&b);
    }
    // println!("b = {b:?}");
}

fn fn_b() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Weak<List>>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(1, RefCell::new(Weak::new())));
    let b = Rc::downgrade(&a);
    let c = b.upgrade();
    println!("c is {c:?}");
    if let Some(s) = &c {
        println!("s is {s:?}");
        if let Cons(i, ref_cell) = s.borrow() {
            println!("i = {i}, ref_cell = {ref_cell:?}");
        } else {
            println!("{Nil:?}")
        }
    }
}

fn fn_c() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    if let Some(parent) = leaf.parent.borrow().upgrade() {
        let value = &parent.value;
        let children = &parent.children;
        println!("value is {value}, children is {children:?}");
    }

    println!("leaf is {leaf:?}");
}
