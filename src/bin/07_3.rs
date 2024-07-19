fn main() {
    fn_a();
    fn_b();
    fn_c();
    fn_d();
    fn_e();
}

mod mod_a {
    // pub，应该就是public的简写
    // 默认是private
    pub fn print(s: &str) {
        println!("{s}");
    }
}

fn fn_a() {
    // 使用::分隔来访问模块内的公有项
    mod_a::print("a");
}

fn fn_b() {
    // 绝对路径方式，crate相当于根
    crate::mod_a::print("crate b");
    // 相对路径方式
    mod_a::print("relative b");
}

fn fn_c() {
    mod mod_c {
        pub fn c() {
            // 使用super可以访问上一层模块中的内容
            super::mod_a::print("super c");
        }
    }
    mod_c::c();
}

fn fn_d() {
    mod mod_d {
        pub struct S {
            // n是公有的
            pub n: i32,
            // s是私有的
            s: String,
        }
        impl S {
            // 定义公有方法
            pub fn new() -> Self {
                Self {
                    n: 10,
                    s: String::from("s"),
                }
            }
            // 定义公有方法
            pub fn to_string(&self) -> String {
                format!("s is {}, n is {}", self.s, self.n)
            }
        }
    }
    let d = mod_d::S::new();
    println!("d.to_string() is {}", d.to_string());
}

fn fn_e() {
    mod mod_e {
        // 枚举设为公有后，项也默认为公有
        #[derive(Debug)]
        pub enum Boolean {
            True,
            // False,
        }
    }
    let e = mod_e::Boolean::True;
    println!("e is {e:?}");
}
