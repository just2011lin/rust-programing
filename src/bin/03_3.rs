fn main() {
    println!("1 + 2 = {}", add(1, 2));
}

// 函数的声明
// fn 函数名(参数: 类型, ...) -> 返回值类型 { 函数体 }
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
