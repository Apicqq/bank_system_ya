//! Вспомогательные элементы для CLI-приложений converter и comparer.
//!
//! Модуль содержит общий разбор имени формата, открытие входных файлов
//! и вызов чтения/записи через реализации `BankFormat`.

use crate::errors::ParseResult;
use crate::format::BankFormat;
use crate::{Transaction, YPBankCsv, YpBankBin, YpBankText};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

/// Открывает файл и возвращает строковую ошибку с путём при неудаче.
///
/// # Errors
///
/// Возвращает ошибку, если файл не удалось открыть.
pub fn open_file(path: &Path) -> Result<File, String> {
    File::open(path).map_err(|error| format!("Could not open file '{}': {}", path.display(), error))
}

/// Формат данных, поддерживаемый CLI-приложениями.
pub enum Format {
    /// Бинарный формат `YPBankBin`.
    Binary,
    /// Табличный формат `YPBankCsv`.
    Csv,
    /// Текстовый формат `YPBankText`.
    Text,
}
impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" | "binary" | "b" | "bytes" => Ok(Self::Binary),
            "csv" | "table" | "c" => Ok(Self::Csv),
            "txt" | "text" | "t" => Ok(Self::Text),
            _ => Err(format!("Unsupported format: {s}")),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Binary => {
                write!(f, "binary")
            }
            Format::Csv => {
                write!(f, "csv")
            }
            Format::Text => {
                write!(f, "text")
            }
        }
    }
}

/// Возвращает следующее значение CLI-аргумента или ошибку, если значение отсутствует.
///
/// # Errors
///
/// Возвращает ошибку, если после флага нет значения.
pub fn next_value<I>(args: &mut I, flag: &'static str) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .ok_or_else(|| format!("missing value for argument: {flag}"))
}

/// Читает транзакции из источника в указанном формате.
///
/// # Errors
///
/// Возвращает ошибку, если данные не соответствуют указанному формату или источник не удалось прочитать.
pub fn read_transactions<R: Read>(reader: R, format: &Format) -> ParseResult<Vec<Transaction>> {
    match format {
        Format::Csv => YPBankCsv::read(reader),
        Format::Text => YpBankText::read(reader),
        Format::Binary => YpBankBin::read(reader),
    }
}

/// Записывает транзакции в приёмник в указанном формате.
///
/// # Errors
///
/// Возвращает ошибку, если транзакции не удалось записать в выбранном формате.
pub fn write_transactions<W: Write>(
    writer: W,
    format: &Format,
    transactions: &[Transaction],
) -> ParseResult<()> {
    match format {
        Format::Csv => YPBankCsv::write(writer, transactions),
        Format::Text => YpBankText::write(writer, transactions),
        Format::Binary => YpBankBin::write(writer, transactions),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_parses_supported_names() {
        assert!(matches!("csv".parse::<Format>(), Ok(Format::Csv)));
        assert!(matches!("table".parse::<Format>(), Ok(Format::Csv)));
        assert!(matches!("text".parse::<Format>(), Ok(Format::Text)));
        assert!(matches!("txt".parse::<Format>(), Ok(Format::Text)));
        assert!(matches!("binary".parse::<Format>(), Ok(Format::Binary)));
        assert!(matches!("bin".parse::<Format>(), Ok(Format::Binary)));
    }

    #[test]
    fn format_rejects_unknown_name() {
        let result = "xml".parse::<Format>();

        assert!(matches!(result, Err(message) if message == "Unsupported format: xml"));
    }

    #[test]
    fn format_display_returns_canonical_name() {
        assert_eq!(Format::Csv.to_string(), "csv");
        assert_eq!(Format::Text.to_string(), "text");
        assert_eq!(Format::Binary.to_string(), "binary");
    }

    #[test]
    fn next_value_returns_following_argument() -> Result<(), String> {
        let mut args = vec!["records.csv".to_string()].into_iter();

        let value = next_value(&mut args, "--input")?;

        assert_eq!(value, "records.csv");
        Ok(())
    }

    #[test]
    fn next_value_returns_error_for_missing_value() {
        let mut args = Vec::<String>::new().into_iter();

        let result = next_value(&mut args, "--input");

        assert!(matches!(
            result,
            Err(message) if message == "missing value for argument: --input"
        ));
    }
}
