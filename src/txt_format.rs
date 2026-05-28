use std::io::{BufRead, BufReader, Read, Write};

use crate::errors::ParserError;
use crate::fields::parse_u64_field;
use crate::{Transaction, TxStatus, TxType, errors::ParseResult, format::BankFormat};

/// Текстовый формат YPBankText.
pub struct YpBankText;

#[derive(Default)]
struct TransactionBuilder {
    tx_id: Option<u64>,
    tx_type: Option<TxType>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<u64>,
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
                parse_u64_field(value, "AMOUNT")?,
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

impl BankFormat for YpBankText {
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>> {
        let reader = BufReader::new(reader);
        parse_transactions(reader)
    }

    fn write<W: Write>(_writer: W, _transactions: &[Transaction]) -> ParseResult<()> {
        todo!()
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
