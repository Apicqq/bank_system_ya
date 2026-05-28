use crate::errors::ParserError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Тип банковской транзакции.
#[derive(Debug, PartialEq, Eq)]
pub enum TxType {
    /// Пополнение счёта.
    Deposit,
    /// Перевод между пользователями.
    Transfer,
    /// Списание со счёта.
    Withdrawal,
}

impl FromStr for TxType {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(Self::Deposit),
            "WITHDRAWAL" => Ok(Self::Withdrawal),
            "TRANSFER" => Ok(Self::Transfer),
            _ => Err(ParserError::InvalidField {
                field: "TX_TYPE",
                value: s.to_string(),
            }),
        }
    }
}

impl Display for TxType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deposit => write!(f, "DEPOSIT"),
            Self::Transfer => write!(f, "TRANSFER"),
            Self::Withdrawal => write!(f, "WITHDRAWAL"),
        }
    }
}

/// Статус обработки банковской транзакции.
#[derive(Debug, PartialEq, Eq)]
pub enum TxStatus {
    /// Транзакция успешно выполнена.
    Success,
    /// Транзакция завершилась ошибкой.
    Failure,
    /// Транзакция ожидает обработки.
    Pending,
}

impl FromStr for TxStatus {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Self::Success),
            "FAILURE" => Ok(Self::Failure),
            "PENDING" => Ok(Self::Pending),
            _ => Err(ParserError::InvalidField {
                field: "STATUS",
                value: s.to_string(),
            }),
        }
    }
}

impl Display for TxStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Failure => write!(f, "FAILURE"),
            Self::Pending => write!(f, "PENDING"),
        }
    }
}
/// Одна запись банковской транзакции YPBank.
#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    /// Уникальный идентификатор транзакции.
    pub tx_id: u64,
    /// Тип транзакции.
    pub tx_type: TxType,
    /// Идентификатор пользователя-отправителя или `0` для пополнения.
    pub from_user_id: u64,
    /// Идентификатор пользователя-получателя или `0` для списания.
    pub to_user_id: u64,
    /// Сумма транзакции в наименьших единицах валюты.
    pub amount: u64,
    /// Unix timestamp транзакции в миллисекундах.
    pub timestamp: u64,
    /// Статус обработки транзакции.
    pub status: TxStatus,
    /// Текстовое описание транзакции.
    pub description: String,
}
