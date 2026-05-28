use crate::errors::{ParseResult, ParserError};

/// Парсит обязательное поле как беззнаковое 64-битное целое число.
///
/// Возвращает [`ParserError::InvalidField`] с именем поля и исходным значением,
/// если значение не является корректным `u64`.
pub fn parse_u64_field(value: &str, field: &'static str) -> ParseResult<u64> {
    value.parse().map_err(|_| ParserError::InvalidField {
        field,
        value: value.to_string(),
    })
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
}
