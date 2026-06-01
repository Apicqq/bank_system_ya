use std::io::{Read, Write};

use crate::{errors::ParseResult, model::Transaction};

/// Общий интерфейс формата банковских транзакций.
///
/// Реализации читают список транзакций из любого источника [`Read`]
/// и записывают его в любой приёмник [`Write`].
pub trait BankFormat {
    /// Читает транзакции из переданного источника.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если источник не удалось прочитать или данные не соответствуют формату.
    fn read<R: Read>(reader: R) -> ParseResult<Vec<Transaction>>;
    /// Записывает транзакции в переданный приёмник.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если данные не удалось записать в приёмник.
    fn write<W: Write>(writer: W, transactions: &[Transaction]) -> ParseResult<()>;
}
