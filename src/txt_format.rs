use std::io::{BufRead, BufReader, Read, Write};

use crate::errors::ParserError;
use crate::fields::{parse_i64_field, parse_u64_field};
use crate::{Transaction, TxStatus, TxType, errors::ParseResult, format::BankFormat};

#[derive(Debug)]
/// Текстовый формат `YPBankText`.
pub struct YpBankText;

impl BankFormat for YpBankText {
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>> {
        let reader = BufReader::new(reader);
        parse_transactions(reader)
    }

    fn write<W: Write>(mut writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        for (index, transaction) in transactions.iter().enumerate() {
            if index > 0 {
                writeln!(writer)?;
            }
            write_transaction(&mut writer, transaction)?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct TransactionBuilder {
    tx_id: Option<u64>,
    tx_type: Option<TxType>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<i64>,
    timestamp: Option<u64>,
    status: Option<TxStatus>,
    description: Option<String>,
}

macro_rules! required_field {
    ($value: expr, $field: literal) => {
        $value.ok_or(ParserError::MissingField($field))?
    };
}

impl TransactionBuilder {
    fn build(self) -> ParseResult<Transaction> {
        Ok(Transaction {
            tx_id: required_field!(self.tx_id, "TX_ID"),
            tx_type: required_field!(self.tx_type, "TX_TYPE"),
            from_user_id: required_field!(self.from_user_id, "FROM_USER_ID"),
            to_user_id: required_field!(self.to_user_id, "TO_USER_ID"),
            amount: required_field!(self.amount, "AMOUNT"),
            timestamp: required_field!(self.timestamp, "TIMESTAMP"),
            status: required_field!(self.status, "STATUS"),
            description: required_field!(self.description, "DESCRIPTION"),
        })
    }
    fn set_field(&mut self, key: &str, value: &str) -> ParseResult<()> {
        match key {
            "TX_ID" => Self::set_once(&mut self.tx_id, parse_u64_field(value, "TX_ID")?, "TX_ID"),
            "TX_TYPE" => Self::set_once(&mut self.tx_type, value.parse::<TxType>()?, "TX_TYPE"),
            "FROM_USER_ID" => Self::set_once(
                &mut self.from_user_id,
                parse_u64_field(value, "FROM_USER_ID")?,
                "FROM_USER_ID",
            ),
            "TO_USER_ID" => Self::set_once(
                &mut self.to_user_id,
                parse_u64_field(value, "TO_USER_ID")?,
                "TO_USER_ID",
            ),
            "AMOUNT" => Self::set_once(
                &mut self.amount,
                parse_i64_field(value, "AMOUNT")?,
                "AMOUNT",
            ),
            "TIMESTAMP" => Self::set_once(
                &mut self.timestamp,
                parse_u64_field(value, "TIMESTAMP")?,
                "TIMESTAMP",
            ),
            "STATUS" => Self::set_once(&mut self.status, value.parse::<TxStatus>()?, "STATUS"),
            "DESCRIPTION" => Self::set_once(
                &mut self.description,
                Self::parse_description(value)?,
                "DESCRIPTION",
            ),
            _ => Err(ParserError::InvalidFormat(format!(
                "unknown field key: {key}"
            ))),
        }
    }

    fn set_once<T>(target: &mut Option<T>, value: T, field: &'static str) -> ParseResult<()> {
        if target.is_some() {
            return Err(ParserError::InvalidFormat(format!(
                "duplicate field: {field}"
            )));
        }

        *target = Some(value);
        Ok(())
    }
    fn parse_description(value: &str) -> ParseResult<String> {
        value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .map(str::to_string)
            .ok_or_else(|| ParserError::InvalidField {
                field: "DESCRIPTION",
                value: value.to_string(),
            })
    }
}

fn parse_transactions<R: BufRead>(reader: R) -> ParseResult<Vec<Transaction>> {
    let mut transactions = Vec::new();
    let mut builder = TransactionBuilder::default();
    let mut has_data = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with('#') {
            continue;
        }

        if line.is_empty() {
            if has_data {
                transactions.push(builder.build()?);
                builder = TransactionBuilder::default();
                has_data = false;
            }

            continue;
        }

        parse_text_line(line, &mut builder)?;
        has_data = true;
    }

    if has_data {
        transactions.push(builder.build()?);
    }

    Ok(transactions)
}

fn parse_text_line(line: &str, builder: &mut TransactionBuilder) -> ParseResult<()> {
    let (key, value) = line.split_once(':').ok_or_else(|| {
        ParserError::InvalidFormat(format!("expected 'KEY: VALUE', got '{line}'"))
    })?;

    builder.set_field(key.trim(), value.trim())
}

fn write_transaction<W: Write>(writer: &mut W, transaction: &Transaction) -> ParseResult<()> {
    writeln!(writer, "TX_ID: {}", transaction.tx_id)?;
    writeln!(writer, "TX_TYPE: {}", transaction.tx_type)?;
    writeln!(writer, "FROM_USER_ID: {}", transaction.from_user_id)?;
    writeln!(writer, "TO_USER_ID: {}", transaction.to_user_id)?;
    writeln!(writer, "AMOUNT: {}", transaction.amount)?;
    writeln!(writer, "TIMESTAMP: {}", transaction.timestamp)?;
    writeln!(writer, "STATUS: {}", transaction.status)?;
    writeln!(writer, "DESCRIPTION: \"{}\"", transaction.description)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transaction() -> Transaction {
        Transaction {
            tx_id: 1_234_567_890_123_456,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 9_876_543_210_987_654,
            amount: 10_000,
            timestamp: 1_633_036_800_000,
            status: TxStatus::Success,
            description: "Terminal deposit".to_string(),
        }
    }

    #[test]
    fn read_parses_text_record() -> ParseResult<()> {
        let input = br#"# Record 1 (Deposit)
TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
DESCRIPTION: "Terminal deposit"
"#;

        let transactions = YpBankText::read(&input[..])?;

        assert_eq!(transactions, vec![sample_transaction()]);

        Ok(())
    }

    #[test]
    fn read_accepts_fields_in_any_order_and_without_final_blank_line() -> ParseResult<()> {
        let input = br#"DESCRIPTION: "Terminal deposit"
STATUS: SUCCESS
TIMESTAMP: 1633036800000
AMOUNT: 10000
TO_USER_ID: 9876543210987654
FROM_USER_ID: 0
TX_TYPE: DEPOSIT
TX_ID: 1234567890123456"#;

        let transactions = YpBankText::read(&input[..])?;

        assert_eq!(transactions, vec![sample_transaction()]);

        Ok(())
    }

    #[test]
    fn read_parses_multiple_records_separated_by_blank_lines() -> ParseResult<()> {
        let input = br#"TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
DESCRIPTION: "Terminal deposit"

# Record 2 (Transfer)
TX_ID: 2312321321321321
TIMESTAMP: 1633056800000
STATUS: FAILURE
TX_TYPE: TRANSFER
FROM_USER_ID: 1231231231231231
TO_USER_ID: 9876543210987654
AMOUNT: 1000
DESCRIPTION: "User transfer"
"#;

        let transactions = YpBankText::read(&input[..])?;

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0], sample_transaction());
        assert_eq!(transactions[1].tx_type, TxType::Transfer);
        assert_eq!(transactions[1].status, TxStatus::Failure);
        assert_eq!(transactions[1].description, "User transfer");

        Ok(())
    }

    #[test]
    fn write_serializes_text_record() -> ParseResult<()> {
        let mut output = Vec::new();
        let transactions = [sample_transaction()];

        YpBankText::write(&mut output, &transactions)?;

        let output = String::from_utf8(output).map_err(|error| {
            ParserError::InvalidFormat(format!("text output is not valid UTF-8: {error}"))
        })?;

        assert_eq!(
            output,
            "TX_ID: 1234567890123456\nTX_TYPE: DEPOSIT\nFROM_USER_ID: 0\nTO_USER_ID: 9876543210987654\nAMOUNT: 10000\nTIMESTAMP: 1633036800000\nSTATUS: SUCCESS\nDESCRIPTION: \"Terminal deposit\"\n"
        );

        Ok(())
    }

    #[test]
    fn write_separates_records_with_blank_line() -> ParseResult<()> {
        let mut output = Vec::new();
        let transactions = [
            sample_transaction(),
            Transaction {
                tx_id: 2_312_321_321_321_321,
                tx_type: TxType::Transfer,
                from_user_id: 1_231_231_231_231_231,
                to_user_id: 9_876_543_210_987_654,
                amount: 1_000,
                timestamp: 1_633_056_800_000,
                status: TxStatus::Failure,
                description: "User transfer".to_string(),
            },
        ];

        YpBankText::write(&mut output, &transactions)?;

        let output = String::from_utf8(output).map_err(|error| {
            ParserError::InvalidFormat(format!("text output is not valid UTF-8: {error}"))
        })?;

        assert!(output.contains("DESCRIPTION: \"Terminal deposit\"\n\nTX_ID: 2312321321321321"));

        Ok(())
    }

    #[test]
    fn write_then_read_preserves_transactions() -> ParseResult<()> {
        let transactions = [
            sample_transaction(),
            Transaction {
                tx_id: 2_312_321_321_321_321,
                tx_type: TxType::Transfer,
                from_user_id: 1_231_231_231_231_231,
                to_user_id: 9_876_543_210_987_654,
                amount: 1_000,
                timestamp: 1_633_056_800_000,
                status: TxStatus::Failure,
                description: "User transfer".to_string(),
            },
        ];
        let mut output = Vec::new();

        YpBankText::write(&mut output, &transactions)?;
        let parsed = YpBankText::read(&output[..])?;

        assert_eq!(parsed, transactions);

        Ok(())
    }

    #[test]
    fn read_returns_missing_field_for_incomplete_record() {
        let input = br"TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
";

        let result = YpBankText::read(&input[..]);

        assert!(matches!(
            result,
            Err(ParserError::MissingField("FROM_USER_ID"))
        ));
    }

    #[test]
    fn read_returns_invalid_format_for_duplicate_field() {
        let input = br#"TX_ID: 1234567890123456
TX_ID: 1234567890123457
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
DESCRIPTION: "Terminal deposit"
"#;

        let result = YpBankText::read(&input[..]);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }

    #[test]
    fn read_returns_invalid_field_for_description_without_quotes() {
        let input = br"TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
DESCRIPTION: Terminal deposit
";

        let result = YpBankText::read(&input[..]);

        match result {
            Err(ParserError::InvalidField { field, value }) => {
                assert_eq!(field, "DESCRIPTION");
                assert_eq!(value, "Terminal deposit");
            }
            other => panic!("expected InvalidField for DESCRIPTION, got {other:?}"),
        }
    }

    #[test]
    fn read_returns_invalid_format_for_unknown_field() {
        let input = br"TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
UNKNOWN: value
";

        let result = YpBankText::read(&input[..]);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }

    #[test]
    fn read_returns_invalid_format_for_line_without_separator() {
        let input = br"TX_ID 1234567890123456
";

        let result = YpBankText::read(&input[..]);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }
}
