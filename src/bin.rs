extern crate blog;
extern crate clap;

use std::path::PathBuf;
use clap::{Arg, App};

use blog::run;

pub fn main() {
    let matches = App::new("Blog compiler")
        .version("1.0")
        .author("Pat Shaughnessy <pat@patshaughnessy.net>")
        .about("Compiles content for patshaughnessy.net")
        .arg(Arg::with_name("DRAFT")
            .help("Compile in draft mode (omit disqus javascript)")
            .short("d")
            .long("draft"))
        .arg(Arg::with_name("INPUT")
            .help("Input directory containing .markdown files")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Target directory where html files will be written")
            .required(true)
            .index(2))
        .get_matches();
    let input = PathBuf::from(matches.value_of("INPUT").unwrap_or(""));
    let output = PathBuf::from(matches.value_of("OUTPUT").unwrap_or(""));
    let draft = matches.is_present("DRAFT");
    match run(input, output, draft) {
        Ok(count) => println!("Compiled {} posts.", count),
        Err(e) => println!("{}", e)
    }
}

