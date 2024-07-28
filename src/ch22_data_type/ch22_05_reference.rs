pub fn main() {
    let a = 1;
    // 指定类型
    let b: &i32 = &a;
    println!("b是a的引用，b={b}");
}
