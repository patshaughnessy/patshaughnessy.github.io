extern crate regex;
extern crate chrono;

use std::fs::File;
use std::path::PathBuf;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::collections::HashMap;
use std::cmp::Ordering;
use chrono::{Datelike, DateTime, NaiveDate, NaiveTime, Utc};
use pulldown_cmark::{Parser, html};
use regex::Regex;
use regex::RegexBuilder;
use regex::Captures;

use std::fs;

use invalid_post_error::InvalidPostError;
use highlight::highlighted_html_for;

#[derive(Debug, Clone, Eq)]
pub struct Post {
    pub path: PathBuf,
    pub url: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub tag: Option<String>,
    pub headers: HashMap<String, String>,
    pub content: String
}

impl Post {
    pub fn from(input_path: &PathBuf) -> Result<Post, InvalidPostError> {
        let lines = read_lines(input_path)?;
        let header_lines = header_lines(&lines);
        let header_map = header_map(&header_lines);
        let title = header_map.get("title").ok_or_else(|| InvalidPostError::new_for_path(input_path, "Missing title"))?;
        let date = date_from_headers(&header_map, input_path)?;
        let url = url_from_headers(&header_map, title, &date);
        let content = html(&lines, &header_map, input_path)?;
        Ok(
            Post {
                path: path_from_url(&url),
                url: url,
                title: title.clone(),
                date: date,
                tag: header_map.get("tag").map(|str| str.clone()),
                headers: header_map.clone(),
                content: content
            }
        )
    }

    pub fn summary(self: &Post) -> String {
        self.content[..400].to_string()
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

fn date_from_headers(header_map: &HashMap<String, String>, input_path: &PathBuf) -> Result<DateTime<Utc>, InvalidPostError> {
    let date_string = header_map.get("date").ok_or_else(|| InvalidPostError::new_for_path(input_path, "Missing date"))?;
    let naive_date = NaiveDate::parse_from_str(date_string, "%Y/%m/%d").map_err(|_| InvalidPostError::new_for_path(&input_path, "Invalid date"))?;
    let midnight = NaiveTime::from_hms_milli(0, 0, 0, 0);
    Ok(DateTime::<Utc>::from_utc(naive_date.and_time(midnight), Utc))
}

fn url_from_headers(header_map: &HashMap<String, String>, title: &String, date: &DateTime<Utc>) -> String {
    let url = header_map.get("url");
    match url {
        Some(url) => url_without_leading_slash(url),
        None      => url_from_title_and_date(title, date)
    }
}

fn url_without_leading_slash(url: &str) -> String {
    let mut char_indices = url.char_indices();
    match char_indices.next() {
        Some((_, chr)) if chr == '/' => String::from(&url[1..]),
        Some((_, _))                 => String::from(&url[0..]),
        None                         => String::from("")
    }
}

fn url_from_title_and_date(title: &String, date: &DateTime<Utc>) -> String {
    let mut path = url_from_date(date);
    path.push('/');
    path.push_str(&slug(title));
    path
}

fn url_from_date(date: &DateTime<Utc>) -> String {
    let mut path = date.year().to_string();
    path.push('/');
    path.push_str(&date.month().to_string());
    path.push('/');
    path.push_str(&date.day().to_string());
    path
}

fn slug(title: &String) -> String {
    lazy_static! {
        static ref AMPERSANDS: Regex = Regex::new(r"&").unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"(\s+)|\.").unwrap();
        static ref OTHER: Regex = Regex::new(r"[^_a-z0-9\-]").unwrap();
    }
    let title = title.to_ascii_lowercase();
    let title = AMPERSANDS.replace_all(&title, "and");
    let title = WHITESPACE.replace_all(&title, "-");
    let title = OTHER.replace_all(&title, "");
    title.to_string()
}

fn path_from_url(url: &String) -> PathBuf {
    let mut path = PathBuf::from(url);
    path.set_extension("html");
    path
}

fn header_map(header_lines: &Vec<String>) -> HashMap<String, String> {
    lazy_static! {
        static ref QUOTED_HEADER_VALUE: Regex = Regex::new("^([a-z]+):\\s+\"([^\"]*)\"").unwrap();
        static ref HEADER_VALUE: Regex = Regex::new("^([a-z]+):\\s+(.*)$").unwrap();
    }
    let mut headers = HashMap::new();
    for line in header_lines {
        if let Some(captures) = QUOTED_HEADER_VALUE.captures(line) {
            let key = String::from(&captures[1]);
            let value = String::from(&captures[2]);
            headers.insert(key, value);
        }
        else if let Some(captures) = HEADER_VALUE.captures(line) {
            let key = String::from(&captures[1]);
            let value = String::from(&captures[2]);
            headers.insert(key, value);
        }
    }
    headers
}

pub fn html(lines: &Vec<String>, header_map: &HashMap<String, String>, input_path: &PathBuf) -> Result<String, InvalidPostError> {
    let html_path = header_map.get("html");
    match html_path {
        Some(html_path) => html_from_file(html_path, input_path),
        None            => html_from_markdown(lines)
    }
}

fn html_from_file(html_path: &String, input_path: &PathBuf) -> Result<String, InvalidPostError> {
    let parent_path = input_path.parent().ok_or_else(|| InvalidPostError::new_for_path(input_path, "Invalid input path"))?;
    let path = parent_path.join(html_path);
    fs::read_to_string(&path).map_err(|_| InvalidPostError::new_for_path(&path, "Unable to read html source file"))
}


fn html_from_markdown(lines: &Vec<String>) -> Result<String, InvalidPostError> {
    let mut markdown = String::new();
    for line in other_lines(lines) {
        markdown.push_str(&line);
        markdown.push('\n');
    }
    let parser = Parser::new(&markdown);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    Ok(with_delim_removed(with_highlighted_code_snippets(&html)))
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
        let trimmed_snippet = snippet.trim().to_string();
        highlighted_html_for(&trimmed_snippet, attributes)
    }).to_string()
}

fn with_delim_removed(html: String) -> String {
    str::replace(&html, "DELIM", "")
}

fn other_lines(lines: &Vec<String>) -> Vec<String> {
    lines.iter().skip_while(|l| is_header(l)).map(|l| l.to_string()).collect()
}

fn header_lines(lines: &Vec<String>) -> Vec<String> {
    lines.iter().take_while(|l| is_header(l)).map(|l| l.to_string()).collect()
}

fn is_header(line: &String) -> bool {
    lazy_static! {
        static ref IS_HEADER: Regex = Regex::new("^[a-z]+:").unwrap();
    }
    line.trim().is_empty() || IS_HEADER.is_match(line)
}

fn read_lines(path: &PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn it_parses_the_title() {
        let post = Post::from(&input_path()).unwrap();
        assert_eq!(post.title, "Learning Rust: If Let vs. Match");
    }

    #[test]
    fn it_parses_the_date() {
        let post = Post::from(&input_path()).unwrap();
        let expected = DateTime::parse_from_rfc3339("2018-01-18T00:00:00+00:00").unwrap();
        assert_eq!(post.date, expected);
    }

    #[test]
    fn it_calculates_an_output_path_and_directory() {
        let post = Post::from(&input_path()).unwrap();
        assert_eq!(post.path, PathBuf::from("2018/1/18/learning-rust-if-let-vs--match.html"));
        assert_eq!(post.url, "2018/1/18/learning-rust-if-let-vs--match");
    }

    #[test]
    fn it_overrides_the_path_when_url_is_specified() {
        let post = Post::from(&input_path2()).unwrap();
        assert_eq!(post.path, PathBuf::from("paperclip-database-storage.html"));
        assert_eq!(post.url, "paperclip-database-storage");
    }

    #[test]
    fn it_returns_an_error_for_an_unexpected_filename() {
        let post = Post::from(&invalid_input_path());
        assert!(post.is_err(), "Post should be invalid");
    }

    #[test]
    fn it_renders_html() {
        let post = Post::from(&input_path()).unwrap();
        let html = fs::read_to_string(rendered_html_path()).unwrap();
        assert_eq!(post.content, html);
    }

    #[test]
    fn it_renders_hard_coded_legacy_html_from_a_file() {
        let post = Post::from(&hard_coded_legacy_input_path()).unwrap();
        let html = fs::read_to_string(hard_coded_legacy_html_path()).unwrap();
        assert_eq!(post.content, html);
    }

    fn input_path() -> PathBuf {
        tests_path().join("2018-01-18-learning-rust-if-let-vs-match.markdown")
    }

    fn input_path2() -> PathBuf {
        tests_path().join("2009-04-14-database-storage-for-paperclip-rewritten-to-use-a-single-table.markdown")
    }

    fn invalid_input_path() -> PathBuf {
        tests_path().join("invalid.markdown")
    }

    fn rendered_html_path() -> PathBuf {
        tests_path().join("learning-rust-if-let-vs--match.html")
    }

    fn hard_coded_legacy_input_path() -> PathBuf {
        tests_path().join("2010-02-20-getting-started-with-ruby-metaprogramming.markdown")
    }

    fn hard_coded_legacy_html_path() -> PathBuf {
        tests_path().join("getting-started-with-ruby-metaprogramming.html")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
