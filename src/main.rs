fn main() {
    // ch15();
    // ch16();
    // ch17();
    // ch19();
    my_try();
}

pub fn ch15() {
    use rust_programing::ch15_smart_pointers as ch_15;
    // ch_15::ch15_01_box::main();
    // ch_15::ch15_02_deref::main();
    // ch_15::ch15_03_drop::main();
    ch_15::ch15_04_rc::main();
    // ch_15::ch15_05_interior_mutable::main();
    // ch_15::ch15_06_reference_circles::main();
}

pub fn ch16() {
    use rust_programing::ch16_concurrency as ch_16;
    // ch_16::ch16_01_threads::main();
    // ch_16::ch16_02_message_passing::main();
    ch_16::ch16_03_shared_state::main();
}

pub fn ch17() {
    use rust_programing::ch17_oop as ch_17;
    // ch_17::ch17_02_trait_objects::main();
    // ch_17::ch17_03_oo_design_paterns::main();
    ch_17::ch17_03_oo_design_paterns::main_a();
}

pub fn ch19() {
    use rust_programing::ch19_advanced_features as ch_19;
    // ch_19::ch19_01_unsafe_rust::main_naked();
    // ch_19::ch19_01_unsafe_rust::main_unsafe_fn();
    // ch_19::ch19_01_unsafe_rust::main_split();
    // ch_19::ch19_01_unsafe_rust::main_c();
    // ch_19::ch19_02_advanced_traits::main_add();
    // ch_19::ch19_02_advanced_traits::main_meters();
    // ch_19::ch19_02_advanced_traits::main_fly();
    // ch_19::ch19_02_advanced_traits::main_baby_name();
    // ch_19::ch19_02_advanced_traits::main_outline_print();
    // ch_19::ch19_02_advanced_traits::main_wrapper();
    // ch_19::ch19_04_advanced_functions_and_clourses::main_fn();
    ch_19::ch19_05_macros::main_my_vec();
}

pub fn my_try() {
    use rust_programing::ch21_ownership;
    // my_try::let_vs_if_let::main_let();
    // my_try::let_vs_if_let::main_if_let();
    // ch21_ownership::move_and_self::main_move();
    // ch21_ownership::move_and_self::main_take();
    // ch21_ownership::ref_and_move::main_use_range();
    // ch21_ownership::move_operation::move_with_let();
    ch21_ownership::move_operation::move_with_if_let();
}
