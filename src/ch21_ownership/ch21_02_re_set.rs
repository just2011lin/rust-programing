pub fn main() {
    let mut a = 10;
    let b = a;
    // 虽然a的值所有权被移走了
    // 但是仍然可以对a进行重新赋值
    a = 20;
    println!("a = {a}, b = {b}");
}
