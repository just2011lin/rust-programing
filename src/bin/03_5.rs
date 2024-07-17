fn main() {
    fn_01();
    fn_02();
    fn_03();
    fn_04();
    fn_05();
    fn_06();
}

fn fn_01() {
    let a = 0;
    // 判断条件必须为布尔值类型
    if a == 0 {
        println!("a为0");
    } else if a == 1 {
        println!("a为1");
    } else {
        println!("a不为0或1");
    }
}

fn fn_02() {
    // rust中没有三元表达式
    // 代替三元表达式
    let b = if true { 1 } else { 0 };
    println!("b is {b}");
}

fn fn_03() {
    let mut c = 0;
    // 'label为标签，可搭配break或continue使用
    'label: loop {
        c += 1;
        if c == 10 {
            continue;
        } else if c == 20 {
            break 'label;
        }
    }
    println!("c is {c}");
}

fn fn_04() {
    let mut a = 0;
    let d = loop {
        a += 1;
        if a == 2 {
            // 终止循环并返回a的值
            break a;
        }
    };
    println!("d is {d}");
}

fn fn_05() {
    let mut e = 10;
    while e >= 0 {
        e -= 1;
    }
    println!("e is {e}");
}

fn fn_06() {
    let a = [1, 2, 3, 4, 5];
    for f in a {
        println!("f is {f}");
    }
}
