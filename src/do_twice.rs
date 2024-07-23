fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn add_one(i: i32) -> i32 {
    i + 1
}

pub fn test_do_twice() {
    let a = do_twice(add_one, 1);
    println!("a = {a}");
}

pub fn test_fn_as_param() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings = {list_of_strings:?}");
}
