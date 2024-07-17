fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    let a = String::from("a");
    // 传递a的引用，引用不会改变所有权
    let r_a = &a;
    println!("a is {a}, r_a is {r_a}");
}

fn fn_b() {
    let mut b = String::from("");
    // 创建b的可变引用
    let m_b = &mut b;
    m_b.push('b');
    println!("b is {b}");
}

// fn fn_c() -> &String {
//     let c = String::from("c");
//     // return &c;
// }
