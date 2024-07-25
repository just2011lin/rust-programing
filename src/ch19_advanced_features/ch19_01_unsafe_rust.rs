pub fn main_naked() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }
}

pub fn main_unsafe_fn() {
    unsafe fn dangerous() {
        let num = 10;
        let r = &num as *const i32;
        println!("unsafe r = {}", *r);
    }
    unsafe {
        dangerous();
    }
}
