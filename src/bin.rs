extern crate blog;

use std::env;
use blog::compile;

pub fn main() {
    if let Some(input_arg) = env::args().nth(1) {
        let dir = env::current_dir().unwrap();
        let input_path = dir.join(input_arg);
        let output_path = dir.join("output_file.txt");
        compile(&input_path, &output_path).expect(&format!("Unable to compile: {:?}\n", input_path));
        println!("Success.");
    } else {
        println!("Usage: blogc INPUT_PATH");
    }
}
