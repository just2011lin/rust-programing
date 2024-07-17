fn main() {
    println!("1 + 2 = {}", add(1, 2));
    fn_01();
}

// 函数的声明
// fn 函数名(参数: 类型, ...) -> 返回值类型 { 函数体 }
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn fn_01() {
    // 大括号创建的表达式
    // rust中，将默认返回未加分号的末尾表达式值
    let y = {
        let x = 1;
        // 相当于
        x + 1
    };
    println!("y is {y}");
}
