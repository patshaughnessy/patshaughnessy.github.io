#![feature(proc_macro_hygiene)]

#[macro_use] extern crate lazy_static;
extern crate pulldown_cmark;
extern crate regex;
extern crate chrono;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Error;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::cmp::Reverse;

mod layout;
mod highlight;
pub mod post;
pub mod post_link;
pub mod invalid_post_error;

use post::Post;
use invalid_post_error::InvalidPostError;

struct CompileInput {
    all_posts: Vec<Post>,
    output_path: PathBuf
}

pub fn run(input_path: PathBuf, output_path: PathBuf) -> Result<usize, InvalidPostError> {
    let paths = fs::read_dir(&input_path)?;
    let all_posts: Result<Vec<Post>, InvalidPostError> =
        paths.filter_map(Result::ok)
             .filter(|f| f.path().extension().and_then(OsStr::to_str) == Some("markdown"))
             .map(|p| Post::from(&output_path, &p.path())).collect();
    let mut all_posts = all_posts?;
    let count = all_posts.len();
    all_posts.sort_by_key(|p| Reverse(p.date));
    let input = CompileInput {all_posts: all_posts, output_path: output_path};
    Ok(input).and_then(compile_posts)
             .and_then(compile_home_page)
             .and_then(compile_rss_feed)
             .map(|_output| count)
}

fn compile_posts(input: CompileInput) -> Result<CompileInput, InvalidPostError> {
    let result: Result<Vec<Post>, InvalidPostError> =
        input.all_posts.iter().map(|p|
            compile_post(&p, &input.all_posts)
        ).collect();
    match result {
        Ok(_) => Ok(input),
        Err(e) => Err(e)
    }
}

pub fn compile_post(post: &Post, all_posts: &Vec<Post>) -> Result<Post, InvalidPostError> {
    let output_path = &post.output_path;
    let output_directory = Path::new(&output_path).parent().ok_or(
        Error::new(ErrorKind::Other, "Invalid output path")
    )?;
    fs::create_dir_all(output_directory)?;
    let mut file = File::create(output_path)?;
    let content = layout::post::render(post, all_posts);
    let content = layout::render(content, Some(&post.title));
    file.write_all(content.as_bytes())?;
    Ok(post.clone())
}

fn compile_home_page(input: CompileInput) -> Result<CompileInput, InvalidPostError> {
    let mut output_path = input.output_path.clone();
    output_path.push("index.html");
    let mut file = File::create(output_path)?;
    let content = layout::home_page::render(&input.all_posts);
    let content = layout::render(content, None);
    file.write_all(content.as_bytes())?;
    Ok(input)
}

fn compile_rss_feed(input: CompileInput) -> Result<CompileInput, InvalidPostError> {
    let mut output_path = input.output_path.clone();
    output_path.push("index.xml");
    let mut file = File::create(output_path)?;
    file.write(
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            layout::rss::render(&input.all_posts)
        ).as_bytes()
    )?;
    Ok(input)
}
