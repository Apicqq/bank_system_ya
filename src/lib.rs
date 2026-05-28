//! Библиотека для чтения, записи и преобразования форматов YPBank.
//!
//! Все форматы преобразуются в общую модель [`Transaction`] и обратно,
//! а ввод и вывод выполняются через стандартные трейты [`std::io::Read`] и [`std::io::Write`].

pub mod bin_format;
pub mod csv_format;
pub mod errors;
pub(crate) mod fields;
pub mod format;
pub mod model;
pub mod txt_format;

pub use model::{Transaction, TxStatus, TxType};
