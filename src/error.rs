use std::io;
use std::num;
use std::result;

pub enum ParseError {
    Integer(num::ParseIntError),
    Float(num::ParseFloatError),
    InvalidStatement
}

pub enum Error {
    IO(io::Error),
    Parse(ParseError),
    LackOfLine,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::Parse(ParseError::Integer(err))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Self {
        Error::Parse(ParseError::Float(err))
    }
}

pub type Result<T> = result::Result<T, Error>;
