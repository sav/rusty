// integration_test.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2022-2024, Savio Sena <savio.sena@gmail.com>

mod common;

#[test]
fn test_integration_minigrep() {
    common::initialize_integration_tests(10);
    unsafe {
        assert_eq!(common::MY_INTEGRATION_TESTS, 10);
    }

    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
}
