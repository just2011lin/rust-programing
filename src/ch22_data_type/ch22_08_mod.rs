pub fn main() {
    // 定义模块
    mod color {
        // 定义模块内结构体
        #[derive(Debug)]
        pub struct RGB(u8, u8, u8);
        // 定义模块内静态常量
        pub static RED: RGB = RGB(255, 0, 0);
        // 定义模块内方法
        pub fn get_blue() -> RGB {
            RGB(0, 0, 255)
        }
        // 定义模块内方法
        pub fn print_rgb(color: &RGB) {
            let RGB(r, g, b) = color;
            println!("r = {r}, g = {g}, b = {b}");
        }
    }
    let a = &color::RED;
    let b = color::get_blue();

    println!("a = {a:?}, b = {b:?}");

    color::print_rgb(a);
}
