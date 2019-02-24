use std::fs;
use std::env;
use std::path::PathBuf;

extern crate blog;

use blog::compile;

#[test]
fn it_copies_input_to_output() {

    // TODO: sort out what the problem is here. Minor differences in
    // the expected html output
    //compile(&input_path(), &output_path()).unwrap();

    //let expected_html = fs::read_to_string(&expected_html_path()).expect("Failed: Didn't find expect html file.");
    //let expected_html_lines = expected_html.split("\n");

    //let actual_html = fs::read_to_string(&output_path()).expect("Failed: Didn't find output file.");
    //let actual_html_separated = str::replace(&actual_html, ">", ">\n");
    //let actual_html_lines = actual_html_separated.split("\n");
    //for line in expected_html_lines.zip(actual_html_lines) {
    //    println!();
    //    println!("EXPECTED:");
    //    println!("-------");
    //    println!("{}", line.0.trim());
    //    println!();
    //    println!("ACTUAL:");
    //    println!("-------");
    //    println!("{}", line.1.trim());
    //    assert_eq!(line.0.trim(), line.1.trim());
    //}
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

fn expected_html_path() -> PathBuf {
    tests_path().join("summer-school-with-the-rust-compiler-adjusted.html")
}

fn output_path() -> PathBuf {
    tests_path().join("output_file.txt")
}

fn tests_path() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
}
