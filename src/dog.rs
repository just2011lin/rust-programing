pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog {}

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

pub fn test_dog() {
    println!("Dog的名字是{}", Dog::baby_name());
    println!("Animal Dog的名字是：{}", <Dog as Animal>::baby_name());
}
