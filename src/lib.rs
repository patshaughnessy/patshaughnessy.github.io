#![feature(proc_macro_hygiene)]

extern crate pulldown_cmark;
use pulldown_cmark::{Parser, html};

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

mod layout;
use layout::render;

pub fn compile(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), std::io::Error> {

    let markdown = fs::read_to_string(input_path)?;
    let mut html = String::new();
    let parser = Parser::new(&markdown);
    html::push_html(&mut html, parser);

    let mut file = File::create(&output_path)?;
    file.write_fmt(format_args!("<html>{}</html>", render(html)))
}
