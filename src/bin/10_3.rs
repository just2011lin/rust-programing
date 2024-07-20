fn main() {
    fn_a();
    fn_b();
}

// 函数签名中的生命周期注解
fn life<'a>(a: &'a str) -> &'a str {
    a
}

fn fn_a() {
    let a = life("a");
    println!("a is {a}");
}

struct S<'a> {
    s: &'a str,
}

impl<'a> S<'a> {
    fn s(&self) -> &'a str {
        self.s
    }
}

fn fn_b() {
    let b = S { s: "123" };
    println!("b.s is {}", b.s());
}
