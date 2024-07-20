fn main() {
    let _ = fn_a("1");
    fn_b();
    fn_c();
}

// 定义泛型
fn fn_a<T>(a: T) -> T {
    a
}

// 定义泛型
struct Point<T> {
    x: T,
    y: T,
}

fn fn_b() {
    let p = Point { x: 1, y: 1 };
    println!("p = ({}, {})", p.x, p.y);
}

#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn fn_c() {
    let c_some = MyOption::Some("1");
    let c_none: MyOption<i32> = MyOption::None;
    println!("c_some is {c_some:?}, c_none is {c_none:?}");
}
