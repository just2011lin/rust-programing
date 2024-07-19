fn main() {
    fn_a();
    fn_b();
}

fn fn_a() {
    // 主动抛出错误
    // 不同于别的语言
    // 未发现rust中有方式可以捕获此错误并阻止程序终止
    // panic!("报错了");
}

fn fn_b() {
    let b = vec![1, 2, 3];
    // 访问数组中不存在的项会导致panic
    println!("b[99] is {}", b[99]);
}
