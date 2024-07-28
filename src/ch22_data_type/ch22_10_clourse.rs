pub fn main() {
    // 什么都不做的闭包
    let a = || {};
    a();

    // 带一个参数的闭包
    let b = |a: &str| {
        println!("a = {a}");
    };

    b("a");

    // 带返回值的闭包
    let c = |a: i32| -> i32 { a + 1 };

    println!("1+1={}", c(1));
}
