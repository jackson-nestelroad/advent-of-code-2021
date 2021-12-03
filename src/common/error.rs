use std::fmt::{Display, Formatter, Result};

pub struct Error {
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Error {
        Error {
            message: message.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Error: {}", self.message)
    }
}
