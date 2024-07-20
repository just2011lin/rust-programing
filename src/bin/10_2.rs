fn main() {
    fn_a();
    fn_b();
    println!("c is {}", fn_c(String::from("hello")));
    println!("d is {}", fn_d(String::from("hello")));
    println!("e is {}", fn_e(String::from("hello")));
    println!("f is {}", fn_f(String::from("abc"), String::new()));
    let g = fn_g();
    println!("g.a() is {}", g.a());
    fn_h();
}

trait A {
    // 定义实现trait A时必要的方法a
    fn a(&self) -> i32;
}

impl A for String {
    fn a(&self) -> i32 {
        self.len() as i32
    }
}

fn fn_a() {
    let s = String::from("hello");
    println!("s.a() is {}", s.a());
}

trait B {
    fn b(&self) -> i32 {
        0
    }
}

impl B for String {}

fn fn_b() {
    let s = String::from("hello");
    println!("s.b() is {}", s.b());
}

// a为实现了Trait A的类型值
fn fn_c(a: impl A) -> i32 {
    a.a()
}

// trait bound语法
// impl A是该语法的简写形式
fn fn_d<T: A>(a: T) -> i32 {
    a.a()
}

// 使用+号，a为同时实现A和B的类型值
fn fn_e(a: impl A + B) -> i32 {
    a.b()
}

// 使用where，用于定义trait比较复杂的情形
fn fn_f<T, U>(a: T, b: U) -> i32
where
    T: A,
    U: B,
{
    a.a() + b.b()
}

// 返回值为实现了A的类型值
fn fn_g() -> impl A {
    String::from("hello world")
}

struct S<T> {
    s: T,
}

// 实现了A的T泛型的S实例
// 才会有如下定义的方法
impl<T: A> S<T> {
    fn add(&self, a: T) -> i32 {
        self.s.a() + a.a()
    }
}

fn fn_h() {
    let s = S {
        s: String::from("hello"),
    };
    let h = s.add(String::from("world"));
    println!("h is {h}");
}
