extern crate blog;
extern crate regex;
extern crate chrono;

use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::env;
use std::ffi::OsStr;

use blog::compile;
use blog::post::Post;

fn run(input_path: PathBuf, output_path: PathBuf) -> Result<(), Box<Error>> {
    let paths = fs::read_dir(input_path)?;
    let markdown_paths = paths.filter_map(Result::ok).filter(|f|
        f.path().extension().and_then(OsStr::to_str) == Some("markdown")
    );
    let posts: Result<Vec<_>, _> = markdown_paths.map(|p|
        Post::from(
            &output_path,
            &p.path()
        )
    ).collect();
    match posts {
        Ok(posts) => {
            println!("Read {} posts.", posts.len());
            let mut result: Result<(), Box<Error>> = Ok(());
            for post in posts {
                //println!("{:?} => {:?}", post.input_path, post.output_path);
                match compile(&post) {
                    Ok(_) => { () },
                    Err(e) => {
                        result = Result::Err(Box::new(e));
                        break
                    }
                }
            };
            result
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

