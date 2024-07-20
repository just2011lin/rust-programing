fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    // 基础使用
    let a = Box::new(5);
    println!("a is {a}");
}

fn fn_b() {
    #[derive(Debug)]
    // 创建递归类型
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let b = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    if let Cons(b, l) = b {
        println!("b is {b}, l is {l:?}");
    }
}
