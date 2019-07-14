extern crate blog;

use std::env;
use std::path::PathBuf;
use blog::run;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: blogc INPUT_PATH OUTPUT_PATH");
    } else {
        match run(PathBuf::from(&args[1]), PathBuf::from(&args[2])) {
            Ok(count) => println!("Compiled {} posts.", count),
            Err(e) => println!("{}", e)
        }
    }
}

