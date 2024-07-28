pub fn main_tuple() {
    let a = (1, false, 'c');
    // 指定元组的类型
    let b: (i32, bool, char) = (2, true, 'd');
    println!("a = {a:?}, b = {b:?}");
}
