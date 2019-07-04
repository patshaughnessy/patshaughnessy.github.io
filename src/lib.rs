#![feature(proc_macro_hygiene)]

#[macro_use] extern crate lazy_static;
extern crate pulldown_cmark;
extern crate regex;
extern crate chrono;

use pulldown_cmark::{Parser, html};

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::io::Error;
use std::io::BufReader;
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

pub fn compile(post: &Post) -> Result<(), Error> {

    // TODO: we probably want to get this from each post, right?
    let lines = read_lines(&post.input_path)?;
    let markdown = text_following_headers(&lines);
    let parser = Parser::new(&markdown);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    let highlighted_html = with_highlighted_code_snippets(&html);

    let output_directory = &post.output_path;
    fs::create_dir_all(output_directory)?;
    let output_path = &post.output_path;
    let mut file = File::create(output_path)?;
    file.write_fmt(format_args!("<html>{}</html>", render(highlighted_html)))
}

fn with_highlighted_code_snippets(html: &String) -> String {
    lazy_static! {
        static ref CODE_SNIPPET: Regex = RegexBuilder::new("<pre[^>]*>(.*?)</pre>")
                                            .dot_matches_new_line(true)
                                            .build()
                                            .unwrap();

    }
    CODE_SNIPPET.replace_all(html, |captures: &Captures| {
        let snippet = captures.get(1).map_or("", |m| m.as_str()).to_string();
        format!("START{}END", highlighted_html_for(&snippet))
    }).into()
}

fn read_lines(path: &PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().map(|l| l.unwrap()).collect()
    )
}

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

    #[test]
    fn it_highlights_each_code_snippet() {
        let lines = read_lines(&snippet_input_path()).unwrap();
        let markdown = text_following_headers(&lines);

        let parser = Parser::new(&markdown);
        let mut html = String::new();
        html::push_html(&mut html, parser);

        let highlighted_html = with_highlighted_code_snippets(&html);

        println!();
        println!("{}", highlighted_html);
        println!();


        //assert_eq!(snippets.len(), 2);
        //assert_eq!(snippets[0], "array = [1, 2, 3]\n  for i in array\n    puts i\n  end");
        //assert_eq!(snippets[1], "$ ruby int-loop.rb\n1\n2\n3");
    }

    #[test]
    fn it_returns_the_markdown_after_the_header_lines() {
        let lines = read_lines(&input_path()).unwrap();
        let markdown = text_following_headers(&lines);
        assert_eq!(markdown, "Learning Rust is hard for everyone, but it’s even worse for me because I’ve\nbeen working with Ruby during past ten years.\n");
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

    #[test]
    fn it_reads_an_array_of_lines() {
        let lines = read_lines(&input_path()).unwrap();
        assert_eq!(lines.len(), 6);
        assert_eq!(lines[5], "been working with Ruby during past ten years.");
    }

    fn snippet_input_path() -> PathBuf {
        tests_path().join("short_input_file_with_code_snippet.txt")
    }

    fn input_path() -> PathBuf {
        tests_path().join("short_input_file.txt")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
