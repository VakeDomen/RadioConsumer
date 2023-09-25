use std::{io, error::Error};



#[derive(Debug)]
pub enum ShoutcastError {
    IoError(io::Error),
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
}

impl From<io::Error> for ShoutcastError {
    fn from(err: io::Error) -> ShoutcastError {
        ShoutcastError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for ShoutcastError {
    fn from(err: std::num::ParseIntError) -> ShoutcastError {
        ShoutcastError::ParseIntError(err)
    }
}

impl From<std::str::Utf8Error> for ShoutcastError {
    fn from(err: std::str::Utf8Error) -> ShoutcastError {
        ShoutcastError::Utf8Error(err)
    }
}

impl std::fmt::Display for ShoutcastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ShoutcastError {}