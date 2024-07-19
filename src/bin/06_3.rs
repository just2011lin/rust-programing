fn main() {
    fn_a();
}

fn fn_a() {
    let n = Some(3);
    // 使用if let匹配并获取值
    if let Some(a) = n {
        println!("a is {a}");
    } else {
        // 这里处理其他情况
    }
}
