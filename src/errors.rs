use csv;
use std::{fmt, io};

/// Ошибка чтения, записи или преобразования данных `YPBank`.
#[derive(Debug)]
pub enum ParserError {
    /// Ошибка ввода-вывода.
    Io(io::Error),
    /// Нарушена структура входного формата.
    InvalidFormat(String),
    /// Поле содержит значение, которое невозможно преобразовать в ожидаемый тип.
    InvalidField {
        /// Название поля с некорректным значением.
        field: &'static str,
        /// Значение, которое не удалось преобразовать.
        value: String,
    },
    /// Обязательное поле отсутствует.
    MissingField(&'static str),
}

/// Результат операций парсинга и сериализации `YPBank`.
pub type ParseResult<T> = Result<T, ParserError>;

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

impl From<csv::Error> for ParserError {
    fn from(value: csv::Error) -> Self {
        if let csv::ErrorKind::Io(io_error) = value.kind() {
            return Self::Io(io::Error::new(io_error.kind(), io_error.to_string()));
        }
        Self::InvalidFormat(value.to_string())
    }
}

impl std::error::Error for ParserError {}
