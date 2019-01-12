
extern crate regex;

use regex::Regex;
use regex::RegexBuilder;

//use std::fs;
//use std::path::Path;
use std::path::PathBuf;
//use std::error::Error;

#[derive(Debug, Clone)]
pub struct Post {
    pub path: PathBuf
}

impl Post {

    fn year(&self) -> String {
        lazy_static! {
            static ref POST_PATH: Regex = Regex::new(r".*(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}).*").unwrap();
        }
        let s = self.path.to_str().unwrap();
        let captures = POST_PATH.captures(s).unwrap();
        captures["year"].to_string()
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn it_has_a_year() {
        let post = Post { path: input_path() };
        assert_eq!(post.year(), "2018");
    }

    fn input_path() -> PathBuf {
        tests_path().join("2018-01-18-learning-rust-if-let-vs-match.markdown")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
