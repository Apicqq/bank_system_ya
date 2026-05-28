use std::io::{Read, Write};

use crate::{errors::ParseResult, format::BankFormat, model::Transaction};

pub struct YPBankTxt;

impl BankFormat for YPBankTxt {
    fn read<R: Read>(_reader: R) -> ParseResult<Vec<Transaction>> {
        todo!()
    }
    fn write<W: Write>(_writer: W, _transactions: &[Transaction]) -> ParseResult<()> {
        todo!()
    }
}
