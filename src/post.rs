extern crate regex;

use regex::Regex;

use std::fs::File;
use std::path::PathBuf;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::collections::HashMap;

use invalid_post_error::InvalidPostError;

#[derive(Debug, Clone)]
pub struct Post {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub output_directory: PathBuf,
    pub headers: HashMap<String, String>
}

impl Post {

    pub fn from(_root_path: &PathBuf, input_path: &PathBuf) -> Result<Post, InvalidPostError> {
        let lines = read_lines(input_path)?;
        let header_lines = header_lines(&lines);
        let header_map = header_map(&header_lines);
        let output_path_tbd = PathBuf::from("TBD output path");
        let output_dir_tbd = PathBuf::from("TBD output dir");
        Ok(
            Post {
                input_path: input_path.clone(),
                output_path: output_path_tbd,
                output_directory: output_dir_tbd,
                headers: header_map
            }
        )
    }

    // Make it pass using rust equivalent of find...
    // But: error handling?
    fn header(&self, name: &str) -> Option<String> {
        // TODO: deal with borrowing here.
        let h = self.headers.clone().into_iter().find(|(key, _)| key == name);
        match h {
            Some(h) => Some(h.1),
            None => None
        }
    }

}

fn read_lines(path: &PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().map(|l| l.unwrap()).collect()
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
        assert_eq!(
            post.header("title").unwrap(),
            "Learning Rust: If Let vs. Match".to_string()
        );
    }

    #[test]
    fn it_parses_the_date() {
        let root_path = PathBuf::from("tests/output");
        let post = Post::from(&root_path, &input_path()).unwrap();
        assert_eq!(
            post.header("date").unwrap(),
            "2018/1/18".to_string()
        );
    }

    //#[test]
    //fn it_calculates_an_output_path() {
    //    let root_path = PathBuf::from("tests/output");
    //    let post = Post::from(&root_path, &input_path()).unwrap();
    //    assert_eq!(post.output_path, PathBuf::from("tests/output/2018/1/18/learning-rust-if-let-vs-match.html"));
    //    assert_eq!(post.output_directory, PathBuf::from("tests/output/2018/1/18"));
    //}

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

    //fn tests_output_path() -> PathBuf {
        //tests_path().join("output")
    //}

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
