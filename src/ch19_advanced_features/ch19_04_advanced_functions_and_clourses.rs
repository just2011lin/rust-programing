pub fn main_fn() {
    fn add_one(i: i32) -> i32 {
        i + 1
    }
    fn do_twice(f: fn(i32) -> i32, i: i32) -> i32 {
        f(i) + f(i)
    }
    let a = do_twice(add_one, 1);
    println!("a = {a}");
}
