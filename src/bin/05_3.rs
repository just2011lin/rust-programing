fn main() {
    fn_a();
    fn_b();
    fn_c();
}

fn fn_a() {
    struct S {
        w: i32,
        h: i32,
    }

    impl S {
        // 定义实例的方法
        fn area(&self) -> i32 {
            self.w * self.h
        }
    }

    let s = S { w: 10, h: 20 };
    // 调用实例的方法
    let a = s.area();
    println!("a is {a}");
}

fn fn_b() {
    struct S {
        w: i32,
        h: i32,
    }
    impl S {
        // 不是用&self，则表明为关联函数
        fn square(size: i32) -> S {
            S { w: size, h: size }
        }
    }

    let b = S::square(20);
    println!("b.w is {}, b.h is {}", b.w, b.h);
}

fn fn_c() {
    struct S {
        w: i32,
        h: i32,
    }
    impl S {
        // 不是用&self，则表明为关联函数
        fn square(size: i32) -> S {
            S { w: size, h: size }
        }
    }
    impl S {
        // 定义实例的方法
        fn area(&self) -> i32 {
            self.w * self.h
        }
    }
    let s = S::square(20);
    let c = s.area();
    println!("c is {c}");
}
