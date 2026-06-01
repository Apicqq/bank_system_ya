//! Библиотека для чтения, записи и преобразования форматов `YPBank`.
//!
//! Все форматы преобразуются в общую модель [`Transaction`] и обратно,
//! а ввод и вывод выполняются через стандартные трейты [`std::io::Read`] и [`std::io::Write`].
#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]
#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]

/// Чтение и запись бинарного формата `YPBankBin`.
pub mod bin_format;
/// Общие вспомогательные элементы для CLI-приложений.
pub mod cli_helpers;
/// Чтение и запись CSV-формата `YPBankCsv`.
pub mod csv_format;
/// Ошибки и результат операций парсинга.
pub mod errors;
pub(crate) mod fields;
/// Общий интерфейс поддерживаемых форматов.
pub mod format;
/// Общая модель банковских транзакций.
pub mod model;
/// Чтение и запись текстового формата `YPBankText`.
pub mod txt_format;

pub use model::{Transaction, TxStatus, TxType};

pub use bin_format::YpBankBin;
pub use csv_format::YPBankCsv;
pub use txt_format::YpBankText;
