//! 会发生所有权转移的操作

pub fn move_with_let() {
    let a = String::from("a");
    // a的所有权转移给了b
    let b = a;
    println!("b = {b}");
}

pub fn move_with_if_let() {
    let a = Some(String::from("a"));
    // a的所有权转移给了b
    if let Some(b) = a {
        println!("b = {b}");
    }
}

// while let

// 引用对象下的属性，不能被转移
// pub fn move_with_attr() {
//     struct Person {
//         name: String,
//     }
//     let a = Person {
//         name: String::from("Lily"),
//     };
//     let b = &a;
//     let c = b.name;
//     println!("c = {c}");
// }

// 函数传参转移所有权

// 闭包中转移所有权

// 属性赋值转移所有权