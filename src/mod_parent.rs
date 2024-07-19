pub mod child;
use crate::mod_other;

pub fn print_parent() {
    mod_other::other_print();
}