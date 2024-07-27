// pub fn main_borrowed() {
//     let a: String = String::from("a");
//     let b: &String = &a;
//     // 编译不可通过
//     // 报错提示a被借用了，所有不能转移所有权
//     let c: String = a;
//     println!("c = {c}, b = {b}");
// }

// 变量的使用范围会影响所有权判定
pub fn main_use_range() {
    let a = String::from("a");
    let b = &a;
    println!("b = {b}");
    // 这里可以编译通过
    // 因为这里超出了b的使用范围
    // 可以理解为b已经不在被使用了
    // 所有可以将a转移给c
    let c = a;
    println!("c = {c}");
}
