use rust_programing::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 200,
                height: 40,
                options: vec!["a".into(), "b".into()],
            }),
            Box::new(Button {
                width: 100,
                height: 40,
                label: "提交".into(),
            }),
        ],
    };

    screen.run();
}
