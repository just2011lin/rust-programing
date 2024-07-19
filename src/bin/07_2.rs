fn main() {
    fn_a();
}

fn fn_a() {
    // mod名推荐使用snake_case格式
    mod mod_a {
        // mod中可以定义方法、结构体、枚举
        pub fn a() {
            println!("mod_a::a方法")
        }
    }
    // 调用mod中的方法
    mod_a::a();
}
