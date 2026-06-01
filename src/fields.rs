use crate::errors::{ParseResult, ParserError};
use std::str::FromStr;

/// Парсит обязательное поле в указанный тип.
///
/// Возвращает [`ParserError::InvalidField`] с именем поля и исходным значением,
/// если значение не удалось преобразовать через [`FromStr`].
fn parse_field<T>(value: &str, field: &'static str) -> ParseResult<T>
where
    T: FromStr,
{
    value.parse().map_err(|_| ParserError::InvalidField {
        field,
        value: value.to_string(),
    })
}

/// Парсит обязательное поле как беззнаковое 64-битное целое число.
///
/// Возвращает [`ParserError::InvalidField`] с именем поля и исходным значением,
/// если значение не является корректным `u64`.
pub(crate) fn parse_u64_field(value: &str, field: &'static str) -> ParseResult<u64> {
    parse_field(value, field)
}

/// Парсит обязательное поле как знаковое 64-битное целое число.
///
/// Возвращает [`ParserError::InvalidField`] с именем поля и исходным значением,
/// если значение не является корректным `i64`.
pub(crate) fn parse_i64_field(value: &str, field: &'static str) -> ParseResult<i64> {
    parse_field(value, field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u64_field_returns_number_for_valid_value() {
        let value = parse_u64_field("42", "TX_ID");

        assert!(matches!(value, Ok(42)));
    }

    #[test]
    fn parse_u64_field_accepts_zero() {
        let value = parse_u64_field("0", "FROM_USER_ID");

        assert!(matches!(value, Ok(0)));
    }

    #[test]
    fn parse_u64_field_returns_invalid_field_for_non_number() {
        let error = parse_u64_field("abc", "AMOUNT").expect_err("expected parsing to fail");

        match error {
            ParserError::InvalidField { field, value } => {
                assert_eq!(field, "AMOUNT");
                assert_eq!(value, "abc");
            }
            other => panic!("expected InvalidField, got {other:?}"),
        }
    }

    #[test]
    fn parse_i64_field_returns_positive_number() {
        let value = parse_i64_field("42", "AMOUNT");

        assert!(matches!(value, Ok(42)));
    }

    #[test]
    fn parse_i64_field_accepts_negative_number() {
        let value = parse_i64_field("-42", "AMOUNT");

        assert!(matches!(value, Ok(-42)));
    }

    #[test]
    fn parse_i64_field_returns_invalid_field_for_non_number() {
        let error = parse_i64_field("abc", "AMOUNT").expect_err("expected parsing to fail");

        match error {
            ParserError::InvalidField { field, value } => {
                assert_eq!(field, "AMOUNT");
                assert_eq!(value, "abc");
            }
            other => panic!("expected InvalidField, got {other:?}"),
        }
    }
}
