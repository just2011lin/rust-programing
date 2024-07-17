fn main() {
    fn_a();
    fn_b();
    fn_c();
}

fn fn_a() {
    // 定义枚举
    enum Gender {
        Male,
        Female,
    }
    // 赋值
    let a_m = Gender::Male;
    let a_f = Gender::Female;
    for a in [a_m, a_f] {
        // 使用match匹配枚举
        match a {
            Gender::Male => print!("a是男性 "),
            Gender::Female => println!("a是女性"),
        }
    }
}

fn fn_b() {
    enum B {
        S(String),
        N(i32),
    }
    let b_s = B::S(String::from("b_s"));
    let b_n = B::N(10);
    for b in [b_s, b_n] {
        match b {
            // 获取枚举中的值
            B::S(s) => print!("b_s is {s},"),
            B::N(n) => println!("b_n is {n}"),
        }
    }
}

fn fn_c() {
    let n = 10;
    match n {
        0 => println!("n is 0"),
        // 匹配剩余所有情况，不需要获取值时可使用_占位符
        other => println!("n is {other} not 0"),
    }
}
