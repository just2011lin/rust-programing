use std::cell::RefCell;

fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    let a = RefCell::new(vec![]);
    println!("a is {a:?},before push");
    a.borrow_mut().push("123");
    println!("a is {a:?}");
}

fn fn_b() {
    let a = RefCell::new(vec![]);
    let mut b_1 = a.borrow_mut();
    // let mut b_2 = a.borrow_mut();
    // let b_3 = a.borrow();
    b_1.push("a");
    // b_2.push("b");
    println!("a is {a:?}");
    // println!("b_3 is {b_3:?}");
}
