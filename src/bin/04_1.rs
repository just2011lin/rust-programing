fn main() {
    fn_a();
    fn_b();
    fn_c();
    fn_d();
    fn_e();
    fn_f();
    fn_g();
}

fn fn_a() {
    // 遵循块作用域规则
    {
        let a = 10;
        println!("a is {a}");
    }
    // a失效
}

fn fn_b() {
    {
        // String是一个在堆上存储的数据
        let b = String::from("Hello World");
        println!("b is {b}");
    }
    // 当b离开作用域时，b将会被清理
}

fn fn_c() {
    let a = String::from("a");
    // 此时将a的所有权转移给了c
    let c = a;
    println!("c is {c}");
}

fn fn_d() {
    let a = String::from("a");
    // 通过clone方法复制数据 ，不转移所有权
    let d = a.clone();
    println!("a is {a}, d is {d}");
}

fn fn_e() {
    let a = 10;
    // 实现了Copy Trait的类型会默认使用复制
    // 而不需要转移所有权
    let e = a;
    println!("a is {a}, e is {e}");
}

fn fn_f() {
    fn f_fn(f: String) {
        println!("f is {f}")
    }

    let a = String::from("a");
    // a的所有权转移给参数a
    f_fn(a)
}

fn fn_g() {
    // 返回值也会转移所有权
    fn g_fn() -> String {
        String::from("g")
    }
    let g = g_fn();
    println!("g is {g}");
}
