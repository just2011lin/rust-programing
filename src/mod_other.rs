use crate::mod_parent;

pub fn other_print() {
    mod_parent::child::print("other_print");
}
