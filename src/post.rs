extern crate regex;

use regex::Regex;
use regex::RegexBuilder;
use regex::Captures;

//use std::fs;
//use std::path::Path;
use std::path::PathBuf;

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Post {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub output_directory: PathBuf
}

impl Post {

    pub fn from(root_path: &PathBuf, input_path: &PathBuf) -> Result<Post, InvalidPostError> {
        lazy_static! {
            static ref POST_PATH: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})-(?P<filename>.*)\.markdown$").unwrap();
        }
        let s = input_path.to_str().unwrap();
        if let Some(captures) = POST_PATH.captures(s) {
            let year = date_value(&captures, "year")?;
            let month = date_value(&captures, "month")?;
            let day = date_value(&captures, "day")?;
            let filename = format!("{}.html", captures["filename"].to_string());
            Ok(
                Post {
                    input_path: input_path.clone(),
                    output_path: root_path.join(&year).join(&month).join(&day).join(&filename),
                    output_directory: root_path.join(&year).join(&month).join(&day)
                }
            )
        } else {
            let message = format!("Invalid post filename: {}", s);
            Err(InvalidPostError::new(&message))
        }
    }
}

fn date_value(captures: &Captures, name: &str) -> Result<String, InvalidPostError> {
    let text_date = &captures[name];
    let int_date = text_date.parse::<i32>()?;
    Ok(int_date.to_string())
}

#[derive(Debug)]
pub struct InvalidPostError {
    details: String
}

impl InvalidPostError {
    fn new(msg: &str) -> InvalidPostError {
        InvalidPostError{details: msg.to_string()}
    }
}

impl fmt::Display for InvalidPostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InvalidPostError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::num::ParseIntError> for InvalidPostError {
    fn from(_: std::num::ParseIntError) -> InvalidPostError {
        InvalidPostError{details: "Unable to parse integer".to_string()}
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn it_calculates_an_output_path() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(post.output_path, PathBuf::from("tests/output/2018/1/18/learning-rust-if-let-vs-match.html"));
        assert_eq!(post.output_directory, PathBuf::from("tests/output/2018/1/18"));
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

    fn tests_output_path() -> PathBuf {
        tests_path().join("output")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
