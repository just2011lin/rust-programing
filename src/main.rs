mod human;
mod meters;
mod point;
fn main() {
    // fn_b();
    // fn_c();
    // fn_d();
    // rust_programing::test_counter();
    // rust_programing::test_dog();
    // rust_programing::test_outline_print();
    // rust_programing::test_do_twice();
    // rust_programing::test_fn_as_param();
    rust_programing::test_my_vec();
}

pub fn fn_b() {
    use point::Point;
    let a = Point { x: 10, y: 10 };
    let b = Point { x: 20, y: 20 };
    let c = a + b;
    println!("c = {c:?}");
}

pub fn fn_c() {
    use meters::{Meters, Millimeters};
    let a = Millimeters(10);
    let b = Meters(1);
    let c = a + b;
    println!("c.0 = {}", c.0);
}

pub fn fn_d() {
    use human::{Human, Pilot, Wizard};
    let a = Human {};
    Pilot::fly(&a);
    Wizard::fly(&a);
    a.fly();
}
