use std::{error, fmt::Display};

pub type Error = Box<dyn error::Error + Send + Sync>;

#[derive(Debug)]
pub enum ParsingError {
    BadHttpVersion,
    BadMethod,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ParsingError {}

//#[derive(Debug)]
//pub enum HttpError {
//    BadRequest,
//    BadMethod,
//}
//
//impl Display for HttpError {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{:?}", self)
//    }
//}
//
//impl error::Error for HttpError {}
