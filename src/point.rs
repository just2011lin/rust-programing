#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

use std::ops::Add;

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.x + rhs.y,
        }
    }
}
