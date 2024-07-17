fn main() {
    fn_a();
    fn_b();
    fn_c();
    fn_d();
    fn_f();
    fn_g();
}

fn fn_a() {
    // 定义结构体
    struct A {
        n: i32,
    }
    // 创建实例
    let a = A { n: 10 };
    // 使用结构体
    println!("a.n is {}", a.n);
}

fn fn_b() {
    struct B {
        n: i32,
    }
    let n = 20;
    // 简写字段n
    let b = B { n };
    println!("b.n is {}", b.n);
}

fn fn_c() {
    struct C {
        s: String,
        n: i32,
    }
    let a = C {
        s: String::from("a"),
        n: 10,
    };
    // ..a 在末尾，使用a中的值补齐剩余字段
    let c = C { n: 20, ..a };
    println!("c.s is {}, c.n is {}", c.s, c.n);
}

fn fn_d() {
    // 定义元祖结构体
    struct D(i32, i32);
    let d = D(10, 20);
    println!("d.0 is {}, d.1 is {}", d.0, d.1);
}

fn fn_f() {
    // 使结构体可以转为Debug打印字符串
    #[derive(Debug)]
    // 类单元结构体
    struct F;
    let f = F;
    println!("f is {f:?}");
}

fn fn_g() {
    let a = 10;
    // dbg宏会打印出：文件名、行列、表达式和结果
    let g = dbg!(a * 2);
    println!("g is {g}");
}