use std::ops::{Deref, DerefMut};

fn main() {
    fn_a();
    fn_b();
    fn_c();
    fn_d();
}

struct S {
    s: i32,
}

// 实现用于解引用的方法
impl Deref for S {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

fn fn_a() {
    let a = S { s: 12 };
    let a = *a;
    println!("*a is {}", a);
}

fn add_2(n: &i32) -> i32 {
    n + 2
}
// 隐式deref强制转换
fn fn_b() {
    let s = S { s: 10 };
    // 如果对s进行解引用会得到&i32的值
    // 则可以使用&s进行传参
    // 过程：先对s调用deref方法，再返回结果值的引用
    let b = add_2(&s);
    println!("b is {b}");
}

impl DerefMut for S {
    // type Target = i32;

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

fn fn_c() {
    let mut c = S { s: 10 };
    // 实现了deref_mut后
    // 可以通过*修改c内部的值
    *c += 2;

    println!("c is {}", *c);
}

fn fn_d() {
    let mut d = 10;
    let p = &mut d;
    *p += 10;
    println!("d is {d}");
}
