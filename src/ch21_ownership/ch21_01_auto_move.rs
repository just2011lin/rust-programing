// pub fn main_struct() {
//     #[derive(Debug)]
//     struct Num {
//         a: i32,
//         b: u32,
//         c: f32,
//     }
//     let a = Num {
//         a: 10,
//         b: 20,
//         c: 1.4,
//     };
//     let b = a;
//     // 即便Num中的所有属性都实现了Copy Trait
//     // 也会转移所有权
//     println!("{a:?}, {b:?}");
// }

pub fn main_copy_struct() {
    #[derive(Debug, Clone)]
    struct Num {
        _a: i32,
    }

    impl Copy for Num {}

    let a = Num { _a: 1 };
    let b = a;
    println!("{a:?}, {b:?}");
}

pub fn main_copy_tuple() {
    let a = (1, false, 'c');
    let b = a;
    println!("{a:?}, {b:?}");
}

// pub fn main_owner_tuple() {
//     let a = (String::from("a"), 1);
//     let b = a;
//     // 当元组中包含未实现Copy Trait的值时
//     // 那么就会发生所有权转移
//     println!("{a:?},{b:?}");
// }

pub fn main_copy_arr() {
    let a = [1, 2, 3];
    let b = a;
    println!("{a:?},{b:?}");
}

// pub fn main_owner_arr() {
//     let a = [String::from("value")];
//     // 将a的所有权转移给了b
//     let b = a;
//     println!("{a:?},{b:?}");
// }
