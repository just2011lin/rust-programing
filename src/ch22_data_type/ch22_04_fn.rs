pub fn main_assign_fn() {
    fn print_a() {
        println!("a");
    }
    // 将函数赋值为另一个名称
    let b: fn() = print_a;
    b();
    // 指定函数作为参数
    fn use_print_a(f: fn()) {
        f();
    }
    // 传递函数
    use_print_a(print_a);
    // 与其他类型会传递所有权不同的是
    // 原来的函数名仍然可用
    print_a();
}
