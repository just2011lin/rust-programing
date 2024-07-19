fn main() {
    fn_a();
    fn_b();
    fn_c();
}

mod parent {
    pub mod child {
        pub fn print(s: &str) {
            println!("{s}");
        }
    }
}

fn fn_a() {
    // 使用use简化路径
    use parent::child::print;
    print("this is a");
}

fn fn_b() {
    // 使用as起一个别名
    use parent::child::print as p;
    p("this is b");
}

fn fn_c() {
    // 使用pub use重新导出print
    pub use parent::child::print;
    print("this is c");
}