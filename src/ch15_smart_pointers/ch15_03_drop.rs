struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("值被丢弃了, data = {}", self.data);
    }
}

pub fn main() {
    let a = CustomSmartPoint {
        data: String::from("abc"),
    };
    drop(a);
}
