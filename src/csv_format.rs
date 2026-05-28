use std::io::{Read, Write};

use crate::{errors::Result, format::BankFormat, model::Transaction};

pub struct YPBankCsv;

impl BankFormat for YPBankCsv {
    fn read<R: Read>(_reader: R) -> Result<Vec<Transaction>> {
        todo!()
    }
    fn write<W: Write>(_writer: W) -> Result<()> {
        todo!()
    }
}
