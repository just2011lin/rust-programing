fn main() {
    fn_a();
}

struct S {
    v: i32,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("Dropping S, v is {}", self.v);
    }
}

fn fn_a() {
    let _ = S { v: 10 };
}
