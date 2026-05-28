use std::io::{Read, Write};

use crate::{errors::ParseResult, model::Transaction};

pub trait BankFormat {
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>>;
    fn write<W: Write>(writer: W) -> ParseResult<()>;
}
