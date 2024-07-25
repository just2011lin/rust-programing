#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y;
    }
}

pub fn main() {
    let mut a = Point { x: 0, y: 0 };
    let b = Box::new(Point { x: 10, y: 10 });
    a.add(&b);
    println!("a = {a:?}");
}
