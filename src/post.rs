extern crate regex;
extern crate chrono;

use regex::Regex;

use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::collections::HashMap;

use chrono::NaiveDate;
use chrono::Datelike;

use invalid_post_error::InvalidPostError;

#[derive(Debug, Clone)]
pub struct Post {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub headers: HashMap<String, String>
}

impl Post {
    pub fn from(root_path: &PathBuf, input_path: &PathBuf) -> Result<Post, InvalidPostError> {
        let lines = read_lines(input_path)?;
        let header_lines = header_lines(&lines);
        let header_map = header_map(&header_lines);
        let path = path_from_headers(&header_map)?;
        Ok(
            Post {
                input_path: input_path.clone(),
                output_path: root_path.join(&path),
                headers: header_map
            }
        )
    }
}

fn path_from_headers(header_map: &HashMap<String, String>) -> Result<PathBuf, InvalidPostError> {
    let url = header_map.get("url");
    match url {
        Some(url) => Ok(path_from_url(url)),
        None      => path_from_date_and_title(header_map)
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

fn path_from_date_and_title(header_map: &HashMap<String, String>) -> Result<PathBuf, InvalidPostError> {
    let path = path_from_date(&header_map)?;
    let slug = slug_from_title(&header_map)?;
    Ok(path.join(slug))
}

fn path_from_date(header_map: &HashMap<String, String>) -> Result<PathBuf, InvalidPostError> {
    let date_string = header_map.get("date").ok_or_else(|| InvalidPostError::new("Missing date"))?;
    let date = NaiveDate::parse_from_str(date_string, "%Y/%m/%d").map_err(|_| InvalidPostError::new("Invalid date"))?;
    let year = PathBuf::from(date.year().to_string());
    let month = PathBuf::from(date.month().to_string());
    let day = PathBuf::from(date.day().to_string());
    Ok(year.join(month).join(day))
}

fn slug_from_title(header_map: &HashMap<String, String>) -> Result<PathBuf, InvalidPostError> {
    lazy_static! {
        static ref AMPERSANDS: Regex = Regex::new(r"&").unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"(\s+)|\.").unwrap();
        static ref OTHER: Regex = Regex::new(r"[^_a-z0-9\-]").unwrap();
    }
    let title = header_map.get("title").ok_or_else(|| InvalidPostError::new("Missing title"))?;
    let title = title.to_ascii_lowercase();
    let title = AMPERSANDS.replace_all(&title, "and");
    let title = WHITESPACE.replace_all(&title, "-");
    let title = OTHER.replace_all(&title, "");
    Ok(PathBuf::from(title.to_string() + ".html"))
}

fn read_lines(path: &PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

// TODO would be nice to use this regex to get the values also
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
        static ref HEADER_VALUE: Regex = Regex::new("^([a-z]+):\\s+(\\S*)").unwrap();
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

    #[test]
    fn it_parses_the_title() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(post.headers.get("title").unwrap(), "Learning Rust: If Let vs. Match");
    }

    #[test]
    fn it_parses_the_date() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(post.headers.get("date").unwrap(), "2018/1/18");
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
        let input_path = tests_path().join("2009-04-14-database-storage-for-paperclip-rewritten-to-use-a-single-table.markdown");
        let post = Post::from(&root_path, &input_path).unwrap();
        assert_eq!(post.output_path, PathBuf::from("tests/output/paperclip-database-storage.html"));
    }

    #[test]
    fn it_returns_an_error_for_an_unexpected_filename() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &invalid_input_path());
        assert!(post.is_err(), "Post should be invalid");
    }

    fn input_path() -> PathBuf {
        tests_path().join("2018-01-18-learning-rust-if-let-vs-match.markdown")
    }

    fn invalid_input_path() -> PathBuf {
        tests_path().join("invalid.markdown")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
