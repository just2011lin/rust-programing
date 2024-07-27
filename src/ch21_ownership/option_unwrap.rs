#[derive(Debug)]
enum Opt {
    Some(String),
    _None,
}

impl Opt {
    fn unwrap(self) -> String {
        match self {
            Self::Some(val) => val,
            Self::_None => panic!("没有值"),
        }
    }
}

pub fn main_move() {
    let s = Opt::Some(String::from("value"));

    // 此操作会移走s中的String
    // 同时使s失效
    let val: String = s.unwrap();

    // 不能再使用s了
    // let b = s;
    println!("val = {val}")
}

pub fn main_as_ref() {
    let s = Some(String::from("value"));

    let val = s.as_ref().unwrap();

    let b = s.as_ref();

    println!("val = {val}, b = {b:?}");
}
