use std::ops::{Deref, DerefMut};

fn main() {
    fn_a();
    fn_b();
    fn_c();
    fn_d();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        Self(v)
    }
}

fn fn_a() {
    let a = MyBox::new(1);
    println!("a.0 = {}", a.0);
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn fn_b() {
    let b = MyBox::new("b");
    println!("b is {}", *b);
}

fn hello(name: &str) {
    println!("hello, {name}");
}

fn fn_c() {
    let c = MyBox::new("Jobs");
    hello(&c);
}

fn fn_d() {
    let mut d = MyBox::new("D");
    *d = "Dd";
    println!("*d = {}", *d);
}
