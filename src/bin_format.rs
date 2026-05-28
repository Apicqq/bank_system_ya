use std::io::{Read, Write};

use crate::{errors::ParseResult, format::BankFormat, model::Transaction};

pub struct YpBankBin;

impl BankFormat for YpBankBin {
    fn read<R: Read>(_reader: R) -> ParseResult<Vec<Transaction>> {
        todo!()
    }
    fn write<W: Write>(_writer: W) -> ParseResult<()> {
        todo!()
    }
}
