fn main() {
    fn_01();
    fn_02();
    fn_03();
    fn_04();
    fn_05();
    fn_06();
}

fn fn_01() {
    let a = 24;
    println!("a is {a}");
}

fn fn_02() {
    let b = 1.3;
    println!("b is {b}");
}

fn fn_03() {
    // bool: false true
    let f: bool = false;
    println!("f is {f}");
}

fn fn_04() {
    // char
    // 注意char的大小为4个字节
    let c: char = 'A';
    println!("c is {c}");
}

fn fn_05() {
    // 声明
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tup is {tup:?}");

    // 解构取值
    let (x, y, z) = tup;
    println!("x,y,z is {x},{y},{z}");

    // 使用.访问
    let z = tup.2;
    println!("z is {z}");
}

fn fn_06() {
    // 声明
    let arr_a = [1, 2, 3, 4];
    println!("arr_a is {arr_a:?}");

    // 声明类型与长度
    let arr_b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr_b is {arr_b:?}");

    // 快捷声明
    let arr_c = [3; 5];
    println!("arr_c is {arr_c:?}");

    // 访问数组元素
    // 访问超出数组长度的索引会报错
    let arr_a_0 = arr_a[0];
    println!("arr_a_0 is {arr_a_0}");
}
