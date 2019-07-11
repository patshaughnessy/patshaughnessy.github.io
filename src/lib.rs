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
use regex::Regex;
use regex::RegexBuilder;
use regex::Captures;

mod layout;
use layout::render;

mod highlight;
use highlight::highlighted_html_for;

pub mod post;
use post::Post;

pub mod invalid_post_error;
use invalid_post_error::InvalidPostError;

use chrono::NaiveDate;

use std::ffi::OsStr;
use std::path::PathBuf;

pub fn run(input_path: PathBuf, output_path: PathBuf) -> Result<Vec<Post>, InvalidPostError> {
    let paths = fs::read_dir(input_path)?;
    let markdown_paths = paths.filter_map(Result::ok).filter(|f|
        f.path().extension().and_then(OsStr::to_str) == Some("markdown")
    );
    let posts: Result<Vec<Post>, InvalidPostError> = markdown_paths.map(|p|
        Post::from(&output_path, &p.path())
    ).collect();
    if let Ok(posts) = posts {
        posts.into_iter().map(|p| compile(&p)).collect()
    } else {
        posts
    }
}

pub fn compile(post: &Post) -> Result<Post, InvalidPostError> {
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
    let title = post.headers.get("title").ok_or_else(|| InvalidPostError::new_for_post(post, "Missing title"))?;
    let date_string = post.headers.get("date").ok_or_else(|| InvalidPostError::new_for_post(post, "Missing date"))?;
    let date = NaiveDate::parse_from_str(date_string, "%Y/%m/%d").map_err(|_| InvalidPostError::new_for_post(post, "Invalid date"))?;
    let formatted_date_string = date.format("%B %e, %Y").to_string();
    let tag = post.headers.get("tag");
    file.write_fmt(
        format_args!(
            "<html>{}</html>",
            render(
                highlighted_html,
                &title,
                &formatted_date_string,
                tag
            )
        )
    )?;
    Ok(post.clone())
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

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn it_highlights_each_code_snippet() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &snippet_input_path()).unwrap();
        let markdown = text_following_headers(&post.lines);

        let parser = Parser::new(&markdown);
        let mut html = String::new();
        html::push_html(&mut html, parser);

        let highlighted_html = with_highlighted_code_snippets(&html);
        //assert_eq!(snippets.len(), 2);
        //assert_eq!(snippets[0], "array = [1, 2, 3]\n  for i in array\n    puts i\n  end");
        //assert_eq!(snippets[1], "$ ruby int-loop.rb\n1\n2\n3");
    }

    #[test]
    fn it_returns_false_without_a_header() {
        let line = "This is not a header line.".to_string();
        assert_eq!(false, is_header(&line));
    }

    #[test]
    fn it_returns_true_with_a_header() {
        let line = "header: This is header!.".to_string();
        assert_eq!(true, is_header(&line));
    }

    fn snippet_input_path() -> PathBuf {
        tests_path().join("short_input_file_with_code_snippet.txt")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
