pub struct Human;

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("飞行员起飞");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("魔女起飞");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("人能飞吗?");
    }
}
