use std::{fmt, io};

#[derive(Debug)]
pub enum ParserError {
    Io(io::Error),
    InvalidFormat(String),
    InvalidField { field: &'static str, value: String },
    MissingField(&'static str),
}

pub type Result<T> = std::result::Result<T, ParserError>;

impl From<io::Error> for ParserError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::InvalidFormat(message) => write!(f, "Invalid format: {message}"),
            Self::InvalidField { field, value } => write!(f, "Invalid value for {field}: {value}"),
            Self::MissingField(field) => write!(f, "Missing required field: {field}"),
        }
    }
}

impl std::error::Error for ParserError {}
