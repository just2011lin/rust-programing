#[derive(Debug)]
pub struct Counter {
    value: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Self { value: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.max {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

pub fn test_counter() {
    let a = Counter::new(10000);
    println!("a = {a:?}");

    for i in a {
        println!("i = {i}");
    }
}
