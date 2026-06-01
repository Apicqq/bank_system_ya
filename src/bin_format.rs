use crate::errors::ParserError;
use crate::{TxStatus, TxType, errors::ParseResult, format::BankFormat, model::Transaction};
use std::io::{BufRead, Cursor, Read, Write};

/// Бинарный формат `YPBankBin`.
pub struct YpBankBin;

const MAGIC: &[u8; 4] = b"YPBN";

impl BankFormat for YpBankBin {
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        let mut reader = std::io::BufReader::new(reader);

        loop {
            if !read_magic_or_eof(&mut reader)? {
                break;
            }
            let record_size = read_u32_be(&mut reader)?;
            let mut body: Vec<u8> = vec![0u8; record_size as usize];
            reader.read_exact(&mut body).map_err(|error| {
                ParserError::InvalidFormat(format!(
                    "Could not read transaction body due to error: {error}"
                ))
            })?;
            let transaction = read_record_body(&body)?;
            transactions.push(transaction);
        }
        Ok(transactions)
    }

    fn write<W: Write>(mut writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        for transaction in transactions {
            let body = transaction_to_body(transaction)?;
            let record_size = u32::try_from(body.len()).map_err(|_| {
                ParserError::InvalidFormat(format!(
                    "binary record body is too large: {} bytes",
                    body.len()
                ))
            })?;
            writer.write_all(MAGIC)?;
            writer.write_all(&record_size.to_be_bytes())?;
            writer.write_all(&body)?;
        }
        Ok(())
    }
}

macro_rules! read_be_number {
    ($fn_name:ident, $type:ty, $size:expr) => {
        fn $fn_name<R: Read>(reader: &mut R) -> ParseResult<$type> {
            let mut bytes = [0u8; $size];
            reader.read_exact(&mut bytes)?;
            Ok(<$type>::from_be_bytes(bytes))
        }
    };
}

fn read_magic_or_eof<R: BufRead>(reader: &mut R) -> ParseResult<bool> {
    let mut buffer = [0u8; 4];
    let mut read_bytes = 0;

    while read_bytes < 4 {
        let number = reader.read(&mut buffer[read_bytes..])?;
        if number == 0 {
            return if read_bytes == 0 {
                Ok(false)
            } else {
                Err(ParserError::InvalidFormat(String::from(
                    "Unexpected EOF while reading magic",
                )))
            };
        }
        read_bytes += number;
    }
    if buffer != *MAGIC {
        return Err(ParserError::InvalidFormat(String::from(
            "Could not read magic byte",
        )));
    }
    Ok(true)
}

read_be_number!(read_u32_be, u32, 4);
read_be_number!(read_u64_be, u64, 8);
read_be_number!(read_i64_be, i64, 8);

fn read_u8<R: Read>(reader: &mut R) -> ParseResult<u8> {
    let mut byte = [0u8; 1];
    reader.read_exact(&mut byte)?;
    Ok(byte[0])
}

fn read_record_body(body: &[u8]) -> ParseResult<Transaction> {
    let mut cursor = Cursor::new(body);

    let tx_id = read_u64_be(&mut cursor)?;
    let tx_type = TxType::from_bin_code(read_u8(&mut cursor)?)?;
    let from_user_id = read_u64_be(&mut cursor)?;
    let to_user_id = read_u64_be(&mut cursor)?;
    let amount = read_i64_be(&mut cursor)?;
    let timestamp = read_u64_be(&mut cursor)?;
    let status = TxStatus::from_bin_code(read_u8(&mut cursor)?)?;
    let desc_len = read_u32_be(&mut cursor)?;

    let mut description_bytes = vec![0u8; desc_len as usize];
    cursor.read_exact(&mut description_bytes)?;

    let description =
        String::from_utf8(description_bytes).map_err(|error| ParserError::InvalidField {
            field: "DESCRIPTION",
            value: error.to_string(),
        })?;
    let description = normalize_binary_description(description);

    let body_len = u64::try_from(body.len())
        .map_err(|_| ParserError::InvalidFormat(String::from("Binary record body is too large")))?;

    if cursor.position() != body_len {
        return Err(ParserError::InvalidFormat(String::from(
            "Binary record body contains trailing bytes",
        )));
    }

    Ok(Transaction {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

fn normalize_binary_description(description: String) -> String {
    description
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_string)
        .unwrap_or(description)
}

fn transaction_to_body(transaction: &Transaction) -> ParseResult<Vec<u8>> {
    let mut body = Vec::new();

    body.extend_from_slice(&transaction.tx_id.to_be_bytes());
    body.push(transaction.tx_type.bin_code());
    body.extend_from_slice(&transaction.from_user_id.to_be_bytes());
    body.extend_from_slice(&transaction.to_user_id.to_be_bytes());
    body.extend_from_slice(&transaction.amount.to_be_bytes());
    body.extend_from_slice(&transaction.timestamp.to_be_bytes());
    body.push(transaction.status.bin_code());

    let description = transaction.description.as_bytes();
    let desc_len = u32::try_from(description.len()).map_err(|_| ParserError::InvalidField {
        field: "DESCRIPTION",
        value: format!("description is too long: {} bytes", description.len()),
    })?;
    body.extend_from_slice(&desc_len.to_be_bytes());
    body.extend_from_slice(description);

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transaction() -> Transaction {
        Transaction {
            tx_id: 1001,
            tx_type: TxType::Transfer,
            from_user_id: 501,
            to_user_id: 502,
            amount: -15_000,
            timestamp: 1_672_534_800_000,
            status: TxStatus::Failure,
            description: "Payment for services".to_string(),
        }
    }

    fn len_to_u32(len: usize) -> u32 {
        u32::try_from(len).expect("test record length must fit into u32")
    }

    fn sample_body() -> Vec<u8> {
        let transaction = sample_transaction();
        let description = transaction.description.as_bytes();
        let mut body = Vec::new();

        body.extend_from_slice(&transaction.tx_id.to_be_bytes());
        body.push(1);
        body.extend_from_slice(&transaction.from_user_id.to_be_bytes());
        body.extend_from_slice(&transaction.to_user_id.to_be_bytes());
        body.extend_from_slice(&transaction.amount.to_be_bytes());
        body.extend_from_slice(&transaction.timestamp.to_be_bytes());
        body.push(1);
        body.extend_from_slice(&len_to_u32(description.len()).to_be_bytes());
        body.extend_from_slice(description);

        body
    }

    fn sample_record() -> Vec<u8> {
        let body = sample_body();
        let mut record = Vec::new();

        record.extend_from_slice(MAGIC);
        record.extend_from_slice(&len_to_u32(body.len()).to_be_bytes());
        record.extend_from_slice(&body);

        record
    }

    #[test]
    fn write_serializes_binary_record() -> ParseResult<()> {
        let mut output = Vec::new();
        let transactions = [sample_transaction()];

        YpBankBin::write(&mut output, &transactions)?;

        assert_eq!(output, sample_record());

        Ok(())
    }

    #[test]
    fn write_then_read_preserves_transactions() -> ParseResult<()> {
        let transactions = [
            sample_transaction(),
            Transaction {
                tx_id: 1002,
                tx_type: TxType::Withdrawal,
                from_user_id: 502,
                to_user_id: 0,
                amount: -1_000,
                timestamp: 1_672_538_400_000,
                status: TxStatus::Pending,
                description: String::new(),
            },
        ];
        let mut output = Vec::new();

        YpBankBin::write(&mut output, &transactions)?;
        let parsed = YpBankBin::read(&output[..])?;

        assert_eq!(parsed, transactions);

        Ok(())
    }

    #[test]
    fn write_empty_transactions_writes_empty_output() -> ParseResult<()> {
        let mut output = Vec::new();

        YpBankBin::write(&mut output, &[])?;

        assert!(output.is_empty());

        Ok(())
    }

    #[test]
    fn write_stores_magic_and_record_size() -> ParseResult<()> {
        let mut output = Vec::new();
        let transactions = [sample_transaction()];

        YpBankBin::write(&mut output, &transactions)?;

        assert_eq!(&output[..MAGIC.len()], MAGIC);

        let mut size_bytes = [0u8; 4];
        size_bytes.copy_from_slice(&output[MAGIC.len()..MAGIC.len() + 4]);
        let record_size = u32::from_be_bytes(size_bytes) as usize;

        assert_eq!(record_size, output.len() - MAGIC.len() - 4);

        Ok(())
    }

    #[test]
    fn read_parses_binary_record() -> ParseResult<()> {
        let transactions = YpBankBin::read(&sample_record()[..])?;

        assert_eq!(transactions, vec![sample_transaction()]);

        Ok(())
    }

    #[test]
    fn read_removes_text_format_quotes_from_description() -> ParseResult<()> {
        let mut transaction = sample_transaction();
        transaction.description = "\"Payment for services\"".to_string();

        let mut record = Vec::new();
        YpBankBin::write(&mut record, &[transaction])?;

        let transactions = YpBankBin::read(&record[..])?;

        assert_eq!(transactions[0].description, "Payment for services");

        Ok(())
    }

    #[test]
    fn read_returns_empty_vec_for_empty_input() -> ParseResult<()> {
        let transactions = YpBankBin::read(&[][..])?;

        assert!(transactions.is_empty());

        Ok(())
    }

    #[test]
    fn read_rejects_invalid_magic() {
        let mut record = sample_record();
        record[0] = b'X';

        let result = YpBankBin::read(&record[..]);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }

    #[test]
    fn read_rejects_invalid_tx_type_code() {
        let mut record = sample_record();
        let tx_type_offset = MAGIC.len() + 4 + 8;
        record[tx_type_offset] = 9;

        let result = YpBankBin::read(&record[..]);

        assert!(matches!(
            result,
            Err(ParserError::InvalidField {
                field: "TX_TYPE",
                ..
            })
        ));
    }

    #[test]
    fn read_rejects_invalid_status_code() {
        let mut record = sample_record();
        let status_offset = MAGIC.len() + 4 + 8 + 1 + 8 + 8 + 8 + 8;
        record[status_offset] = 9;

        let result = YpBankBin::read(&record[..]);

        assert!(matches!(
            result,
            Err(ParserError::InvalidField {
                field: "STATUS",
                ..
            })
        ));
    }

    #[test]
    fn read_rejects_body_with_trailing_bytes() {
        let mut body = sample_body();
        body.push(0);
        let mut record = Vec::new();

        record.extend_from_slice(MAGIC);
        record.extend_from_slice(&len_to_u32(body.len()).to_be_bytes());
        record.extend_from_slice(&body);

        let result = YpBankBin::read(&record[..]);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }

    #[test]
    fn read_rejects_truncated_body() {
        let mut record = sample_record();
        record.pop();

        let result = YpBankBin::read(&record[..]);

        assert!(matches!(result, Err(ParserError::InvalidFormat(_))));
    }
}
