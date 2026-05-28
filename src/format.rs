use std::io::{Read, Write};

use crate::{errors::ParseResult, model::Transaction};

/// Общий интерфейс формата банковских транзакций.
///
/// Реализации читают список транзакций из любого источника [`Read`]
/// и записывают его в любой приёмник [`Write`].
pub trait BankFormat {
    /// Читает транзакции из переданного источника.
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>>;
    /// Записывает транзакции в переданный приёмник.
    fn write<W: Write>(writer: W, transaction: &[Transaction]) -> ParseResult<()>;
}
