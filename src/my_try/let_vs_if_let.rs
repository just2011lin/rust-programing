// use std::{thread, time::Duration};

struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("丢弃 ({}, {})", self.x, self.y);
    }
}

impl Point {
    fn sum(&self) -> Option<i32> {
        Some(self.x + self.y)
    }
}

pub fn main_let() {
    let some_p = Some(Point { x: 1, y: 1 });
    let s = some_p.unwrap().sum().unwrap();
    println!("s = {s}");
}

pub fn main_if_let() {
    let some_p = Some(Point { x: 1, y: 1 });
    if let Some(s) = some_p.unwrap().sum() {
        // 在打印这句话之前，会先将some_p中的Point实例丢弃掉
        println!("s = {s}");
    };
}
