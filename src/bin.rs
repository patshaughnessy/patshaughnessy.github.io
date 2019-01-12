extern crate blog;

#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs;
//use std::path::Path;
//use std::path::PathBuf;
use std::error::Error;

use std::env;

//use regex::Regex;
//use regex::RegexBuilder;

use blog::compile;

mod post;
use post::Post;

fn run(input: &String, output: &String) -> Result<(), Box<Error>> {
    //let _attr = fs::metadata(input)?;
    //let _attr = fs::metadata(output)?;

    let paths = fs::read_dir(input)?;
    let posts: Vec<Post> = paths.map(|p| Post { path: p.unwrap().path() }).collect();
    println!("Read {} posts.", posts.len());
    if let Some(first) = posts.first() {
        println!("First post: {:?}", first);
    }
    Ok(())
}

//pub fn main() {
//    let args: Vec<String> = env::args().collect();
//    if args.len() != 3 {
//        println!("Usage: blogc INPUT_PATH OUTPUT_PATH");
//    } else if let Err(e) = run(&args[1], &args[2]) {
//        println!("{}", e);
//    }
//}

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
