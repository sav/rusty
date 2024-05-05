// mod.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2022-2024, Savio Sena <savio.sena@gmail.com>

pub static mut MY_INTEGRATION_TESTS: i32 = 0;

pub fn initialize_integration_tests(initialization_value: i32) {
    unsafe {
        MY_INTEGRATION_TESTS = initialization_value;
    }
}
