mod mod_other;
mod mod_parent;

fn main() {
    // println!("Hello, world!");
    mod_parent::child::print("this is main");
    mod_other::other_print();
    mod_parent::print_parent();
    println!("CHILD is {}", mod_parent::child::CHILD);
}
