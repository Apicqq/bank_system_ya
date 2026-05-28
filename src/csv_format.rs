use crate::errors::ParserError;
use crate::{TxStatus, TxType, fields::parse_u64_field};
use crate::{errors::ParseResult, format::BankFormat, model::Transaction};
use std::io::{Read, Write};

/// Формат CSV для записей YPBank.
///
/// Поддерживает чтение и запись CSV-файлов со строгим заголовком
/// `TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION`.
pub struct YPBankCsv;

const CSV_FIELD_COUNT: usize = 8;
static HEADER: [&str; CSV_FIELD_COUNT] = [
    "TX_ID",
    "TX_TYPE",
    "FROM_USER_ID",
    "TO_USER_ID",
    "AMOUNT",
    "TIMESTAMP",
    "STATUS",
    "DESCRIPTION",
];

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
                amount: parse_u64_field(&record[4], "AMOUNT")?,
                timestamp: parse_u64_field(&record[5], "TIMESTAMP")?,
                status: record[6].parse::<TxStatus>()?,
                description: record[7].to_string(),
            };
            transactions.push(transaction)
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
