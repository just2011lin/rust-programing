fn main() {
    // ch15();
    // ch16();
    // ch17();
    // ch19();
    // ch21();
    ch22();
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

pub fn ch21() {
    use rust_programing::ch21_ownership;
    // my_try::let_vs_if_let::main_let();
    // my_try::let_vs_if_let::main_if_let();
    // ch21_ownership::move_and_self::main_move();
    // ch21_ownership::move_and_self::main_take();
    // ch21_ownership::ref_and_move::main_use_range();
    // ch21_ownership::move_operation::move_with_let();
    // ch21_ownership::move_operation::move_with_if_let();
    // ch21_ownership::ch21_01_auto_move::main_copy_tuple();
    // ch21_ownership::ch21_01_auto_move::main_copy_struct();
    ch21_ownership::ch21_01_auto_move::main_copy_arr();
}

pub fn ch22() {
    use rust_programing::ch22_data_type as ch22;

    // ch22::ch22_01_scalar::main_integer();
    // ch22::ch22_01_scalar::main_float();
    // ch22::ch22_01_scalar::main_bool();
    // ch22::ch22_01_scalar::main_char();

    // ch22::ch22_02_tuple::main_tuple();

    // ch22::ch22_03_array::main_array();

    // ch22::ch22_04_fn::main_assign_fn();

    // ch22::ch22_05_reference::main();
    // ch22::ch22_05_reference::main_ref_ref();

    // ch22::ch22_06_struct::main_normal_struct();
    // ch22::ch22_06_struct::main_tuple_struct();
    // ch22::ch22_06_struct::main_unit_struct();

    // ch22::ch22_07_enum::main_no_value_enum();
    // ch22::ch22_07_enum::main_with_value_enum();

    // ch22::ch22_08_mod::main();

    // ch22::ch22_09_vec_string_hashmap::main_vec();
    // ch22::ch22_09_vec_string_hashmap::main_string();
    // ch22::ch22_09_vec_string_hashmap::main_hash_map();

    // ch22::ch22_10_clourse::main();

    // ch22::ch22_11_smart_ppointers::main_at();
    ch22::ch22_11_smart_ppointers::main_box();
}
