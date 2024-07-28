//！标量类型
pub fn main_integer() {
    // 默认为i32类型
    let a = 10;
    // 指定i8类型
    let b: i8 = 10;
    // 设定超出范围的值会报错
    // let c: i8 = 2555;
    // 可使用_分隔数字
    let c: u32 = 1024_255_888;
    println!("a = {a}, b = {b}, c = {c}");
}

pub fn main_float() {
    // 默认为f64类型
    let a = 1.2;
    // 指定f32类型
    let b: f32 = 2.4;
    println!("a = {a}, b = {b}");
}

pub fn main_bool() {
    let a = false;
    // 指定bool类型
    let b: bool = true;
    println!("a = {a}, b = {b}");
}

pub fn main_char() {
    let a: char = 'c';
    // 指定char类型
    // char类型的存储空间为四个字节
    let b: char = '♥';
    println!("a = {a}, b = {b}");
}
