pub fn main() {
    let a = 1;
    // 指定类型
    let b: &i32 = &a;
    println!("b是a的引用，b={b}");
}

pub fn main_ref_ref() {
    let a: i32 = 1;
    let b: &&i32 = &&a;
    let c: &i32 = b;
    println!("c = {c}");
}
