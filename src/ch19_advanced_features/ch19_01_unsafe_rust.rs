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

pub fn main_split() {
    use std::slice;
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();
        assert!(mid <= len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut nums = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut nums, 3);
    println!("a = {a:?}, b = {b:?}");
}

pub fn main_c() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("-3的绝对值为：{}", abs(-3));
    }
}
