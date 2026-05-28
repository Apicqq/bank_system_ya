use std::io::{Read, Write};

use crate::{errors::Result, model::Transaction};

pub trait BankFormat {
    fn read<R: Read>(reader: R) -> Result<Vec<Transaction>>;
    fn write<W: Write>(writer: W) -> Result<()>;
}
