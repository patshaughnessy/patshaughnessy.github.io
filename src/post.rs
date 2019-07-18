extern crate regex;
extern crate chrono;

use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
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

use invalid_post_error::InvalidPostError;
use highlight::highlighted_html_for;

#[derive(Debug, Clone, Eq)]
pub struct Post {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub url: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub tag: Option<String>,
    pub headers: HashMap<String, String>,
    pub content: String
}

impl Post {
    pub fn from(root_path: &PathBuf, input_path: &PathBuf) -> Result<Post, InvalidPostError> {
        let lines = read_lines(input_path)?;
        // TODO: partition to get other lines also
        let header_lines = header_lines(&lines);
        let header_map = header_map(&header_lines);
        let title = header_map.get("title").ok_or_else(|| InvalidPostError::new_for_path(input_path, "Missing title"))?;
        let tag = header_map.get("tag").map(|str| str.clone());
        let date = date_from_headers(&header_map, input_path)?;
        let path = path_from_headers(&header_map, title, &date)?;
        let url_string = path.as_path().as_os_str().to_str().ok_or_else(|| InvalidPostError::new_for_path(input_path, "Invalid path"))?;
        let absolute_url = format!("/{}", url_string);
        Ok(
            Post {
                input_path: input_path.clone(),
                output_path: root_path.join(&path),
                url: absolute_url,
                title: title.clone(),
                date: date,
                tag: tag,
                headers: header_map.clone(),
                content: html(lines)
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

fn path_from_headers(header_map: &HashMap<String, String>, title: &String, date: &DateTime<Utc>) -> Result<PathBuf, InvalidPostError> {
    let url = header_map.get("url");
    match url {
        Some(url) => Ok(path_from_url(url)),
        None      => path_from_title_and_date(title, date)
    }
}

fn path_from_url(url: &str) -> PathBuf {
    let mut path = if Path::new(url).has_root() {
        PathBuf::from(&url[1..])
    } else {
        PathBuf::from(url)
    };
    path.set_extension("html");
    path
}

fn path_from_title_and_date(title: &String, date: &DateTime<Utc>) -> Result<PathBuf, InvalidPostError> {
    let path = path_from_date(date)?;
    let slug = slug(title)?;
    Ok(path.join(slug))
}

fn path_from_date(date: &DateTime<Utc>) -> Result<PathBuf, InvalidPostError> {
    let year = PathBuf::from(date.year().to_string());
    let month = PathBuf::from(date.month().to_string());
    let day = PathBuf::from(date.day().to_string());
    Ok(year.join(month).join(day))
}

fn slug(title: &String) -> Result<PathBuf, InvalidPostError> {
    lazy_static! {
        static ref AMPERSANDS: Regex = Regex::new(r"&").unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"(\s+)|\.").unwrap();
        static ref OTHER: Regex = Regex::new(r"[^_a-z0-9\-]").unwrap();
    }
    let title = title.to_ascii_lowercase();
    let title = AMPERSANDS.replace_all(&title, "and");
    let title = WHITESPACE.replace_all(&title, "-");
    let title = OTHER.replace_all(&title, "");
    Ok(PathBuf::from(title.to_string() + ".html"))
}

pub fn html(lines: Vec<String>) -> String {
    let markdown = lines.iter().skip_while(|l| is_header(l))
                               .fold(String::new(), |str, line|
                                    str+&format!("{}\n", line)
                               );
    let parser = Parser::new(&markdown);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    with_delim_removed(
        with_highlighted_code_snippets(&html)
    )
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
    }).to_string()
}

fn with_delim_removed(html: String) -> String {
    str::replace(&html, "DELIM", "")
}

fn read_lines(path: &PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

fn is_header(line: &String) -> bool {
    lazy_static! {
        static ref IS_HEADER: Regex = Regex::new("^[a-z]+:").unwrap();
    }
    line.trim().is_empty() || IS_HEADER.is_match(line)
}

fn header_lines(lines: &Vec<String>) -> Vec<String> {
    lines.iter().take_while(|l| is_header(l)).map(|l| l.to_string()).collect()
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

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn it_parses_the_title() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(post.title, "Learning Rust: If Let vs. Match");
    }

    #[test]
    fn it_parses_the_date() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(post.date, DateTime<Utc>::parse_from_str("2018/1/18", "%Y/%m/%d").unwrap());
    }

    #[test]
    fn it_calculates_an_output_path_and_directory() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(post.output_path, PathBuf::from("tests/output/2018/1/18/learning-rust-if-let-vs--match.html"));
    }

    #[test]
    fn it_overrides_the_path_when_url_is_specified() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path2()).unwrap();
        assert_eq!(post.output_path, PathBuf::from("tests/output/paperclip-database-storage.html"));
    }

    #[test]
    fn it_returns_an_error_for_an_unexpected_filename() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &invalid_input_path());
        assert!(post.is_err(), "Post should be invalid");
    }

    #[test]
    fn it_renders_html() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        let html = fs::read_to_string(rendered_html_path()).unwrap();
        assert_eq!(post.content(), html);
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
        tests_path().join("learning-rust-if-let-vs-match.html")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
