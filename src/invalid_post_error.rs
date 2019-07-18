use std::fmt;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
enum InvalidPostErrorContext {
    PathContext(PathBuf)
}

#[derive(Debug)]
pub struct InvalidPostError {
    context: Option<InvalidPostErrorContext>,
    details: String
}

impl InvalidPostError {
    pub fn new_for_path(path: &PathBuf, msg: &str) -> InvalidPostError {
        let context = InvalidPostErrorContext::PathContext(path.clone());
        InvalidPostError {
            context: Some(context),
            details: msg.to_string()
        }
    }

    pub fn new(msg: &str) -> InvalidPostError {
        InvalidPostError {
            context: None,
            details: msg.to_string()
        }
    }
}

impl fmt::Display for InvalidPostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref c) = self.context {
            write_context(c, &self.details, f)
        } else {
            write!(f, "{}", self.details)
        }
    }
}

fn write_context(context: &InvalidPostErrorContext, details: &String, f: &mut fmt::Formatter) -> fmt::Result {
    match context {
        InvalidPostErrorContext::PathContext(path) => {
            write!(f, "{} in {}", details, path.to_str().unwrap())
        }
    }
}

impl Error for InvalidPostError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::num::ParseIntError> for InvalidPostError {
    fn from(_: std::num::ParseIntError) -> InvalidPostError {
        InvalidPostError::new("Unable to parse integer")
    }
}

impl From<std::io::Error> for InvalidPostError {
    fn from(_: std::io::Error) -> InvalidPostError {
        InvalidPostError::new("TBD IO Error")
    }
}
