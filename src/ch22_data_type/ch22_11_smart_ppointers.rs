pub fn main_at() {
    #[derive(Debug)]
    enum List<'a> {
        Cons(i32, &'a List<'a>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Cons(1, &Nil);
    let b = Cons(2, &a);
    println!("b = {b:?}");

    let mut c = &b;

    while let Cons(i, ref_a) = c {
        println!("i == {i}, ref_a = {ref_a:?}");
        c = ref_a;
    }
}

pub fn main_box() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let a = Cons(1, Box::new(Nil));
    let b = Cons(2, Box::new(a));
    println!("b = {b:?}");

    let mut c = b;
    while let Cons(i, ref_box) = c {
        println!("i = {i}");
        c = *ref_box;
    }
}
