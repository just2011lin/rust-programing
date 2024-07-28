pub fn main_vec() {
    let mut v = Vec::new();
    v.push("Hello");
    v.push("World");
    let s = v.join(",");
    println!("s = {s}");
}

pub fn main_string() {
    let mut s = String::new();
    s.push_str("Hello, World");
    println!("s = {s}");
}

pub fn main_hash_map() {
    use std::collections::HashMap;

    let mut week_day_map = HashMap::new();
    week_day_map.insert(0, "天");
    week_day_map.insert(1, "一");
    // ..
    let week_day = week_day_map.get(&0);
    println!("week_day = {week_day:?}");
}
