use std::cell::RefCell;

fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    let a = RefCell::new(5);
    *(a.borrow_mut()) = 15;
    println!("a = {:?}", a.borrow());
}

fn fn_b() {
    let mut a = 10;
    let b = &mut a;
    *b = 20;
    println!("b = {b}");
}
