use std::cell::RefCell;

pub fn main() {
    let sent_messages = RefCell::new(vec![]);
    sent_messages.borrow_mut().push("123");

    let messages = sent_messages.borrow();

    let ms: &Vec<&str> = messages.as_ref();

    println!("ms = {ms:?}");
}
