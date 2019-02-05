extern crate regex;

use regex::Regex;
use regex::RegexBuilder;

//use std::fs;
//use std::path::Path;
use std::path::PathBuf;

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Post {
    pub input_path: PathBuf,
    pub output_path: PathBuf
}

impl Post {

    //pub fn from(root_path: &PathBuf, input_path: &PathBuf) -> Option<Post> {
    pub fn from(root_path: &PathBuf, input_path: &PathBuf) -> Result<Post, InvalidPostError> {
        lazy_static! {
            static ref POST_PATH: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})-(?P<filename>.*)").unwrap();
        }
        let s = input_path.to_str().unwrap();
        // TODO:
        // - Return a custom error if the regex fails here.
        // - And look for an "else" style syntax
        if let Some(captures) = POST_PATH.captures(s) {
            // Why aren't the [] returning optionals?
            let year = captures["year"].to_string();
            let month = captures["month"].to_string();
            let day = captures["day"].to_string();
            let filename = captures["filename"].to_string();
            Ok(
                Post {
                input_path: input_path.clone(),
                output_path: root_path.join(year).join(month).join(day).join(filename)
                }
            )
        } else {
            Err(InvalidPostError::new("Invalid post filename"))
        }
    }
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

// a test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(),InvalidPostError> {
    if yes {
        Err(InvalidPostError::new("borked"))
    } else {
        Ok(())
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
        assert_eq!(post.output_path, PathBuf::from("tests/output/2018/01/18/learning-rust-if-let-vs-match.markdown"));
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
