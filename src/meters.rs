use std::ops::Add;
pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}
