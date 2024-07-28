pub fn main_normal_struct() {
    struct Struct {
        n: i32,
        s: String,
    }
    let a = Struct {
        n: 10,
        s: String::from("s"),
    };
    println!("a.n = {}, a.s = {}", a.n, a.s);
}

pub fn main_tuple_struct() {
    struct Struct(i32, String);
    let a = Struct(10, String::from("a"));
    println!("a = ({},{})", a.0, a.1);
}

pub fn main_unit_struct() {
    #[derive(Debug)]
    struct Struct;
    let a = Struct;
    println!("a = {a:?}");
}
