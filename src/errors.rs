#[derive(Debug)]
pub enum MyError {
    DependencyNotSatisfied,
    PathNotInitialized,
    PathNotValid,
    UnexpectedError {
        text: String,
    },
}

use std::fmt;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::DependencyNotSatisfied =>
                write!(f, "Required dependency is not satisfied"),

            MyError::PathNotInitialized =>
                write!(f, "PATH environment variable is not initialized"),

            MyError::PathNotValid =>
                write!(f, "PATH is present but contains invalid data"),

            MyError::UnexpectedError { text } =>
                write!(f, "Unexpected error: {text}"),
        }
    }
}
use std::error::Error;

impl Error for MyError {}

use std::io;
impl From<MyError> for io::Error {
    fn from(err: MyError) -> Self {
        match err {
            MyError::DependencyNotSatisfied =>
                io::Error::new(io::ErrorKind::NotFound, err),

            MyError::PathNotInitialized =>
                io::Error::new(io::ErrorKind::NotFound, err),

            MyError::PathNotValid =>
                io::Error::new(io::ErrorKind::InvalidData, err),

            MyError::UnexpectedError { .. } =>
                io::Error::new(io::ErrorKind::Other, err),
        }
    }
}
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound =>
                MyError::PathNotInitialized,

            std::io::ErrorKind::InvalidData =>
                MyError::PathNotValid,

            _ =>
                MyError::UnexpectedError {
                    text: err.to_string(),
                },
        }
    }
}
