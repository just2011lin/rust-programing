#[derive(Debug)]
struct Size {
    width: i32,
    height: i32,
}

impl Size {
    fn area(self) -> i32 {
        self.width * self.height
    }
}

pub fn main_move() {
    #[derive(Debug)]
    struct Rectangle {
        s: Size,
    }

    let a = Rectangle {
        s: Size {
            width: 10,
            height: 20,
        },
    };
    // a.s.area()方法中需要移入a.s的所有权
    // 此操作同时也会移走a的所有权
    let s = a.s.area();
    // 此时a便不可访问了
    // let b = a;
    println!("s = {s}");
}

pub fn main_take() {
    #[derive(Debug)]
    struct Rectangle {
        s: Option<Size>,
    }

    let mut a = Rectangle {
        s: Some(Size {
            width: 10,
            height: 20,
        }),
    };
    // 使用take取出Some中的值，并会留下一个None
    // 此操作不会使a不可访问
    let s = a.s.take().unwrap().area();
    // 如此还能访问a，只不过a.s的值变为了None
    let b = a;
    println!("s = {s}, b = {b:?}");
}
