pub fn main_my_vec() {
    #[macro_export]
    macro_rules! my_vec {
        ($( $x:expr ), *) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let v = my_vec![1, 2, 3];
    println!("v = {v:?}");
}
