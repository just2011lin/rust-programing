use std::fs::File;
use std::io;

fn main() {
    fn_a();
    let _ = fn_b();
    let _ = fn_c();
}

fn fn_a() {
    // 打开文件时，可能会产生错误
    // f为一个Result类型的值
    let f = File::open("1.text");
    // println!("f is {f:?}");
    match f {
        Ok(file) => println!("打开文件成功, {file:?}"),
        Err(e) => println!("打开文件失败: {e:?}"),
    }
    // 当ok时，unwrap返回值，err时，unwrap会panic
    // expect同理
    // let file = f.unwrap();
    // println!("file is {file:?}");
}

// 定义Result类型返回值
fn fn_b() -> Result<File, io::Error> {
    let f = File::open("1.text");
    let ok;
    match f {
        Ok(file) => ok = file,
        // 返回错误
        Err(e) => return Err(e),
    }
    // 成功处理的返回值
    Ok(ok)
}

fn fn_c() -> Result<File, io::Error> {
    // ?的操作是：当ok时返回值，当err时返回错误并退出当前函数
    let f = File::open("1.text")?;
    Ok(f)
}
