fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    let s = String::from("Hello World");
    // 创建s的字符串slice
    // 0..5 是range语法
    let a = &s[0..5];
    println!("a is {a}");
}

fn fn_b() {
    let a = [1, 2, 3, 4, 5];
    // 数组的slice
    let b = &a[1..3];
    println!("b is {b:?}");
}
