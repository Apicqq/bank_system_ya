use crate::errors::ParserError;
use crate::{TxStatus, TxType, errors::ParseResult, format::BankFormat, model::Transaction};
use std::io::{BufRead, Cursor, Read, Write};

macro_rules! read_be_number {
    ($fn_name:ident, $type:ty, $size:expr) => {
        fn $fn_name<R: Read>(reader: &mut R) -> ParseResult<$type> {
            let mut bytes = [0u8; $size];
            reader.read_exact(&mut bytes)?;
            Ok(<$type>::from_be_bytes(bytes))
        }
    };
}

/// Бинарный формат YPBankBin.
pub struct YpBankBin;

const MAGIC: &[u8; 4] = b"YPBN";

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
        read_bytes += number
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

    if cursor.position() as usize != body.len() {
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
                    "Could not read transaction body due to error: {}",
                    error
                ))
            })?;
            let transaction = read_record_body(&body)?;
            transactions.push(transaction);
        }
        Ok(transactions)
    }
    fn write<W: Write>(_writer: W, _transactions: &[Transaction]) -> ParseResult<()> {
        todo!()
    }
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
        body.extend_from_slice(&(description.len() as u32).to_be_bytes());
        body.extend_from_slice(description);

        body
    }

    fn sample_record() -> Vec<u8> {
        let body = sample_body();
        let mut record = Vec::new();

        record.extend_from_slice(MAGIC);
        record.extend_from_slice(&(body.len() as u32).to_be_bytes());
        record.extend_from_slice(&body);

        record
    }

    #[test]
    fn read_parses_binary_record() -> ParseResult<()> {
        let transactions = YpBankBin::read(&sample_record()[..])?;

        assert_eq!(transactions, vec![sample_transaction()]);

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
        record.extend_from_slice(&(body.len() as u32).to_be_bytes());
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
