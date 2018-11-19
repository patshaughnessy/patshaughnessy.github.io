use std::fs;
use std::env;
use std::path::PathBuf;

extern crate blog;

use blog::compile;

#[test]
fn it_copies_input_to_output() {
    compile(&input_path(), &output_path()).unwrap();
    let copied = fs::read_to_string(&output_path()).expect("Failed: Didn't find output file.");
    assert_eq!(copied, "This is an input file.\n");
}

#[test]
fn it_returns_error_for_invalid_input_path() {
    let invalid_input_path = tests_path().join("invalid.txt");
    let contents = compile(&invalid_input_path, &output_path());
    assert!(contents.is_err());
}

fn input_path() -> PathBuf {
    tests_path().join("input_file.txt")
}

fn output_path() -> PathBuf {
    tests_path().join("output_file.txt")
}

fn tests_path() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
}
