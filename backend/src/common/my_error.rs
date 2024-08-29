use std::fmt::{Display, Formatter};
use std::fs::write;
use std::error::Error;
#[derive(Debug)]
pub enum MyError{
    SimpleError,
    DetaildError(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            MyError::SimpleError => write!(f, "An error occured"),
            MyError::DetaildError(ref msg) => write!(f, "MyError : {}", msg),
        }
    }
}

impl Error for MyError{

}

