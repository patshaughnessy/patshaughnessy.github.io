extern crate blog;

use std::env;
use blog::compile;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: blogc INPUT_PATH OUTPUT_PATH");
    } else {
        let input = &args[1];
        let output = &args[2];
        let dir = env::current_dir().unwrap();
        let input_path = dir.join(input);
        let output_path = dir.join(output);
        compile(&input_path, &output_path).expect(&format!("Unable to compile: {:?}\n", input_path));
        println!("Success.");
    }
}
