fn main() {
    // ch15();
    ch16();
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
