fn main() {
    fn_a();
    fn_b();
    fn_c();
}

fn fn_a() {
    #[derive(Debug)]
    enum A {
        X,
        Y,
    }
    let a_x = A::X;
    let a_y = A::Y;
    println!("a_x is {a_x:?},a_y is {a_y:?}");
}

fn fn_b() {
    #[derive(Debug)]
    enum B {
        X(i32),
        S(String),
    }
    let b_x = B::X(10);
    let b_s = B::S(String::from("b_s"));
    for b in [b_x, b_s] {
        match b {
            B::X(x) => println!("x is {x}"),
            B::S(s) => println!("s is {s}"),
        }
    }
}

fn fn_c() {
    // 创建Option枚举的值
    let c: Option<i32> = Some(3);
    // 从Option枚举中取值
    match c {
        Some(c) => println!("c is {c}"),
        None => println!("c is none"),
    }
}
