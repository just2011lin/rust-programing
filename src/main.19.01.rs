
fn main() {
    // unsafe {
    //     fn_a();
    // }

    fn_b();
}

pub unsafe fn fn_a() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1 = {r1:?}");
    println!("r2 = {r2:?}");
    println!("*r1 = {}", *r1);
    *r2 = 10;
    println!("*r2 = {}", *r2);
}

pub fn fn_b() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("a = {a:?}, b = {b:?}");
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    use std::slice;
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
