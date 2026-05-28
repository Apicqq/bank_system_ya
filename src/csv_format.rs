use std::io::{Read, Write};

use crate::{TxStatus, TxType, fields::parse_u64_field};
use crate::{errors::ParseResult, format::BankFormat, model::Transaction};

pub struct YPBankCsv;

impl BankFormat for YPBankCsv {
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);

        let mut transactions: Vec<Transaction> = Vec::new();

        for record_result in csv_reader.records() {
            let record = record_result?;

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
    fn write<W: Write>(_writer: W) -> ParseResult<()> {
        todo!()
    }
}
