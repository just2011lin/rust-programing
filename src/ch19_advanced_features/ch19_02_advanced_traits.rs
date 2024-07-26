pub fn main_add() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    let a = Point { x: 1, y: 1 };
    let b = Point { x: 2, y: 2 };
    let c = a + b;
    println!("c = {c:?}");
}

pub fn main_meters() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(rhs.0 * 1000 + self.0)
        }
    }

    let a = Millimeters(8);
    let b = Meters(2);
    let c = a + b;
    println!("c = {c:?}");
}

pub fn main_fly() {
    trait Pilot {
        fn fly(&self) {
            println!("飞行员驾驶飞机进行飞行");
        }
    }
    trait Wizard {
        fn fly(&self) {
            println!("巫师使用法杖的力量进行飞行");
        }
    }
    struct Human {}
    impl Human {
        fn fly(&self) {
            println!("人类无法靠自身实现飞行");
        }
    }
    impl Pilot for Human {}
    impl Wizard for Human {}
    let h = Human {};
    h.fly();
    Pilot::fly(&h);
    Wizard::fly(&h);
}

pub fn main_baby_name() {
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("Dog中的名字是: {}", Dog::baby_name());
    println!("Animal中Dog的名字是: {}", <Dog as Animal>::baby_name());
}

pub fn main_outline_print() {
    trait OutlinePrint: std::fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("**{}**", "*".repeat(len));
            println!("* {} *", " ".repeat(len));
            println!("* {output} *");
            println!("* {} *", " ".repeat(len));
            println!("**{}**", "*".repeat(len));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
    let p = Point { x: 10, y: 20 };
    p.outline_print();
}

pub fn main_wrapper() {
    struct Wrapper(Vec<String>);
    impl std::ops::Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl std::ops::DerefMut for Wrapper {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.join(","))
        }
    }
    let mut w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    w.push(String::from("Perfect"));
    println!("w = {w}");
}
