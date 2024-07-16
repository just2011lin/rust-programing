fn main() {
    // fn_01();
    // fn_02();
    // fn_03();
    // fn_04();
    fn_05();
}

// fn fn_01() {
//     let a = 10;
//     // 错误：let声明的变量是不可变的
//     a = 20; // Error：cannot assign twice to immutable variable `a`
// }

// fn fn_02() {
//     let mut a = 10;
//     println!("a is {a}");
//     // let mut 声明可变变量
//     a = 20;
//     println!("a is {a}");
// }

// fn fn_03() {
//     const SECONDS_OF_ONE_MINUTE: i32 = 60;
//     println!("SECONDS_OF_ONE_MINUTE is {SECONDS_OF_ONE_MINUTE}");
// }

// fn fn_04() {
//     // 常量必须要注明类型
//     const SECONDS_STR: &str = "SECONDS";
//     println!("SECONDS_STR is {SECONDS_STR}");
// }

fn fn_05() {
    let a = 10;
    println!("a is {a}");
    // 可使用let重新声明变量a
    // 之前的变量a会被隐藏
    let a = "20";
    print!("a is {a}");
}
