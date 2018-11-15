extern crate blog;
use std::process;

use blog::compile;

pub fn main() {
    if let Err(err) = compile("a", "b") {
        println!("{}", err);
        process::exit(1);
    }
    println!("Success.");
}
