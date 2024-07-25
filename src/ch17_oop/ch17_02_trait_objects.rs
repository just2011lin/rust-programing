// use crate::ch17_oop::ch17_02_gui::{Button, Draw, Screen};
use super::ch17_02_gui::{Button, Draw, Screen};
use std::fmt::Display;

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select-box: {self}");
    }
}

impl Display for SelectBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.options.join(",");
        write!(f, "({}, {}, {})", self.width, self.height, s)
    }
}

pub fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 20,
                label: String::from("确认"),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 40,
                options: vec![String::from("好的"), String::from("不好")],
            }),
        ],
    };
    screen.run();
}
