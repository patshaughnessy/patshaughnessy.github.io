use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct InvalidPostError {
    details: String
}

impl InvalidPostError {
    pub fn new(msg: &str) -> InvalidPostError {
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

impl From<std::io::Error> for InvalidPostError {
    fn from(_: std::io::Error) -> InvalidPostError {
        InvalidPostError{details: "TBD IO Error".to_string()}
    }
}
