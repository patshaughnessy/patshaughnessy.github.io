#![feature(proc_macro_hygiene)]

#[macro_use] extern crate lazy_static;
extern crate pulldown_cmark;
extern crate regex;
extern crate chrono;

use pulldown_cmark::{Parser, html};

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Error;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::cmp::Reverse;
use regex::Regex;
use regex::RegexBuilder;
use regex::Captures;

mod layout;
use layout::render_article;
use layout::render_home_page;

mod highlight;
use highlight::highlighted_html_for;

pub mod post;
use post::Post;

pub mod invalid_post_error;
use invalid_post_error::InvalidPostError;

use chrono::NaiveDate;

use std::ffi::OsStr;
use std::path::PathBuf;

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
             .map(|output| count)
}

fn compile_posts(input: CompileInput) -> Result<CompileInput, InvalidPostError> {
    let compiled_posts: Result<Vec<Post>, InvalidPostError> =
        input.all_posts.iter().map(|p|
            compile_post(&p, &input.all_posts)
        ).collect();
    let compiled_posts = compiled_posts?;
    Ok(input)
}

// It's a bit weird that post encapsulates the output path.
// and then we need to get output directory.
// would be cleaner to save just the relative path in post and
// pass in the CompileInput here
pub fn compile_post(post: &Post, all_posts: &Vec<Post>) -> Result<Post, InvalidPostError> {
    let markdown = text_following_headers(&post.lines); // Should be a Post method
    let parser = Parser::new(&markdown);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    let highlighted_html = with_highlighted_code_snippets(&html);
    let output_path = &post.output_path;
    let output_directory = Path::new(&output_path).parent().ok_or(
        Error::new(ErrorKind::Other, "Invalid output path")
    )?;
    fs::create_dir_all(output_directory)?;
    let mut file = File::create(output_path)?;
    file.write_fmt(
        format_args!(
            "<html>{}</html>",
            render_article(
                highlighted_html,
                &post,
                &all_posts
            )
        )
    )?;
    let mut home_page_file = File::create("index.html")?;
    home_page_file.write_fmt(
        format_args!(
            "<html>{}</html>",
            render_home_page(&all_posts)
        )
    )?;
    Ok(post.clone())
}

fn compile_home_page(input: CompileInput) -> Result<CompileInput, InvalidPostError> {
    let mut output_path = input.output_path.clone();
    output_path.push("index.html");
    let mut file = File::create(output_path)?;
    file.write_fmt(
        format_args!(
            "<html>{}</html>",
            render_home_page(&input.all_posts)
        )
    )?;
    Ok(input)
}

fn with_highlighted_code_snippets(html: &String) -> String {
    lazy_static! {
        static ref CODE_SNIPPET: Regex = RegexBuilder::new("<pre([^>]*)>(.*?)</pre>")
                                            .dot_matches_new_line(true)
                                            .build()
                                            .unwrap();

    }
    CODE_SNIPPET.replace_all(html, |captures: &Captures| {
        let attributes = captures.get(1).map(|m| m.as_str().to_string());
        let snippet = captures.get(2).map_or("", |m| m.as_str()).to_string();
        format!("{}", highlighted_html_for(&snippet, attributes))
    }).into()
}

// These two methods should go into Post
fn text_following_headers(lines: &Vec<String>) -> String {
    lines.iter().skip_while(|l| is_header(l))
                .fold(String::new(), |str, line|
                      str+&format!("{}\n", line)
                )
}

fn is_header(line: &String) -> bool {
    lazy_static! {
        static ref IS_HEADER: Regex = Regex::new("^[a-z]+:").unwrap();
    }
    line.trim().is_empty() || IS_HEADER.is_match(line)
}

//#[cfg(test)]
//mod tests {
//
//    use super::*;
//    use std::env;
//    use std::path::PathBuf;
//
//    #[test]
//    fn it_highlights_each_code_snippet() {
//        let root_path = PathBuf::from("tests/output");
//        let post = Post::from(&root_path, &snippet_input_path()).unwrap();
//        let markdown = text_following_headers(&post.lines);
//
//        let parser = Parser::new(&markdown);
//        let mut html = String::new();
//        html::push_html(&mut html, parser);
//
//        let highlighted_html = with_highlighted_code_snippets(&html);
//        //assert_eq!(snippets.len(), 2);
//        //assert_eq!(snippets[0], "array = [1, 2, 3]\n  for i in array\n    puts i\n  end");
//        //assert_eq!(snippets[1], "$ ruby int-loop.rb\n1\n2\n3");
//    }
//
//    #[test]
//    fn it_returns_false_without_a_header() {
//        let line = "This is not a header line.".to_string();
//        assert_eq!(false, is_header(&line));
//    }
//
//    #[test]
//    fn it_returns_true_with_a_header() {
//        let line = "header: This is header!.".to_string();
//        assert_eq!(true, is_header(&line));
//    }
//
//    fn snippet_input_path() -> PathBuf {
//        tests_path().join("short_input_file_with_code_snippet.txt")
//    }
//
//    fn tests_path() -> PathBuf {
//        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
//    }
//}
