extern crate blog;

#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs;
//use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use std::env;


//use regex::Regex;
//use regex::RegexBuilder;

use blog::compile;

mod post;
use post::Post;
use post::InvalidPostError;

fn run(input_path: PathBuf, output_path: PathBuf) -> Result<(), Box<Error>> {
    let paths = fs::read_dir(input_path)?;
    let posts: Result<Vec<_>, _> = paths.map(
        |p| Post::from(
                &output_path,
                &p.unwrap().path()
            )
        ).collect();
    match posts {
        Ok(posts) => {
            println!("Read {} posts.", posts.len());
            Ok(())
        },
        Err(e) => {
            Result::Err(Box::new(e))
        }
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: blogc INPUT_PATH OUTPUT_PATH");
    } else if let Err(e) = run(
        PathBuf::from(&args[1]),
        PathBuf::from(&args[2])
    ) {
        println!("{}", e);
    }
}

//        compile(&input_path, &output_path).expect(&format!("Unable to compile: {:?}\n", input_path));
