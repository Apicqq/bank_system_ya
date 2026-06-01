use crate::errors::ParserError;
use crate::fields::parse_i64_field;
use crate::{TxStatus, TxType, fields::parse_u64_field};
use crate::{errors::ParseResult, format::BankFormat, model::Transaction};
use std::io::{Read, Write};

#[derive(Debug)]
/// Формат CSV для записей `YPBank`.
///
/// Поддерживает чтение и запись CSV-файлов со строгим заголовком
/// `TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION`.
pub struct YPBankCsv;

const CSV_FIELD_COUNT: usize = 8;
const HEADER: [&str; CSV_FIELD_COUNT] = [
    "TX_ID",
    "TX_TYPE",
    "FROM_USER_ID",
    "TO_USER_ID",
    "AMOUNT",
    "TIMESTAMP",
    "STATUS",
    "DESCRIPTION",
];

impl BankFormat for YPBankCsv {
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);

        let mut transactions: Vec<Transaction> = Vec::new();

        for record_result in csv_reader.records() {
            let record = record_result?;

            validate_csv_length(&record)?;
            let transaction = Transaction {
                tx_id: parse_u64_field(&record[0], "TX_ID")?,
                tx_type: record[1].parse::<TxType>()?,
                from_user_id: parse_u64_field(&record[2], "FROM_USER_ID")?,
                to_user_id: parse_u64_field(&record[3], "TO_USER_ID")?,
                amount: parse_i64_field(&record[4], "AMOUNT")?,
                timestamp: parse_u64_field(&record[5], "TIMESTAMP")?,
                status: record[6].parse::<TxStatus>()?,
                description: record[7].to_string(),
            };
            transactions.push(transaction);
        }
        Ok(transactions)
    }
    fn write<W: Write>(writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(writer);

        writer.write_record(HEADER)?;

        for transaction in transactions {
            writer.write_record(transaction_to_csv_record(transaction))?;
        }

        writer.flush()?;
        Ok(())
    }
}

fn transaction_to_csv_record(transaction: &Transaction) -> [String; 8] {
    [
        transaction.tx_id.to_string(),
        transaction.tx_type.to_string(),
        transaction.from_user_id.to_string(),
        transaction.to_user_id.to_string(),
        transaction.amount.to_string(),
        transaction.timestamp.to_string(),
        transaction.status.to_string(),
        transaction.description.clone(),
    ]
}

fn validate_csv_length(record: &csv::StringRecord) -> ParseResult<()> {
    if record.len() != CSV_FIELD_COUNT {
        return Err(ParserError::InvalidFormat(format!(
            "CSV record must contain exactly {CSV_FIELD_COUNT} fields, got {}",
            record.len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_invalid_field(
        result: ParseResult<Vec<Transaction>>,
        expected_field: &str,
        expected_value: &str,
    ) {
        match result.expect_err("expected CSV parsing to fail") {
            ParserError::InvalidField { field, value } => {
                assert_eq!(field, expected_field);
                assert_eq!(value, expected_value);
            }
            other => panic!("expected InvalidField, got {other:?}"),
        }
    }

    fn sample_transaction() -> Transaction {
        Transaction {
            tx_id: 1001,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 501,
            amount: 50_000,
            timestamp: 1_672_531_200_000,
            status: TxStatus::Success,
            description: "Initial account funding".to_string(),
        }
    }

    #[test]
    fn read_parses_csv_records() -> ParseResult<()> {
        let input = br#"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding"
1002,TRANSFER,501,502,15000,1672534800000,FAILURE,"Payment for services, invoice #123"
"#;

        let transactions = YPBankCsv::read(&input[..])?;

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0], sample_transaction());
        assert_eq!(transactions[1].tx_id, 1002);
        assert_eq!(transactions[1].tx_type, TxType::Transfer);
        assert_eq!(
            transactions[1].description,
            "Payment for services, invoice #123"
        );

        Ok(())
    }

    #[test]
    fn write_serializes_csv_records() -> ParseResult<()> {
        let mut output = Vec::new();
        let transactions = [sample_transaction()];

        YPBankCsv::write(&mut output, &transactions)?;

        let output = String::from_utf8(output).map_err(|error| {
            ParserError::InvalidFormat(format!("CSV output is not valid UTF-8: {error}"))
        })?;

        assert_eq!(
            output,
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,Initial account funding\n"
        );

        Ok(())
    }

    #[test]
    fn write_then_read_preserves_transactions() -> ParseResult<()> {
        let transactions = [
            sample_transaction(),
            Transaction {
                tx_id: 1002,
                tx_type: TxType::Transfer,
                from_user_id: 501,
                to_user_id: 502,
                amount: 15_000,
                timestamp: 1_672_534_800_000,
                status: TxStatus::Failure,
                description: "Payment for services, invoice #123".to_string(),
            },
        ];
        let mut output = Vec::new();

        YPBankCsv::write(&mut output, &transactions)?;
        let parsed = YPBankCsv::read(&output[..])?;

        assert_eq!(parsed, transactions);

        Ok(())
    }

    #[test]
    fn read_returns_invalid_field_for_invalid_tx_id() {
        let input = br#"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
abc,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding"
"#;

        let result = YPBankCsv::read(&input[..]);

        assert_invalid_field(result, "TX_ID", "abc");
    }

    #[test]
    fn read_returns_invalid_field_for_invalid_tx_type() {
        let input = br#"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,UNKNOWN,0,501,50000,1672531200000,SUCCESS,"Initial account funding"
"#;

        let result = YPBankCsv::read(&input[..]);

        assert_invalid_field(result, "TX_TYPE", "UNKNOWN");
    }

    #[test]
    fn read_returns_invalid_field_for_invalid_status() {
        let input = br#"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,BROKEN,"Initial account funding"
"#;

        let result = YPBankCsv::read(&input[..]);

        assert_invalid_field(result, "STATUS", "BROKEN");
    }

    #[test]
    fn validate_csv_length_rejects_record_with_wrong_field_count() {
        let record = csv::StringRecord::from(vec!["1001", "DEPOSIT"]);

        let result = validate_csv_length(&record);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }
}
