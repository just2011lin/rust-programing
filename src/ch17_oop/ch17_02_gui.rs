use std::fmt::Display;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.width, self.height, self.label)
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button: {self}");
    }
}
