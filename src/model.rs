use crate::errors::ParserError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Тип банковской транзакции.
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxType {
    /// Пополнение счёта.
    Deposit = 0,
    /// Перевод между пользователями.
    Transfer = 1,
    /// Списание со счёта.
    Withdrawal = 2,
}

impl TxType {
    /// Возвращает числовой код типа транзакции для бинарного формата YPBankBin.
    pub(crate) fn bin_code(&self) -> u8 {
        match self {
            Self::Deposit => 0,
            Self::Transfer => 1,
            Self::Withdrawal => 2,
        }
    }

    /// Преобразует числовой код бинарного формата YPBankBin в тип транзакции.
    pub(crate) fn from_bin_code(code: u8) -> Result<Self, ParserError> {
        match code {
            0 => Ok(Self::Deposit),
            1 => Ok(Self::Transfer),
            2 => Ok(Self::Withdrawal),
            _ => Err(ParserError::InvalidField {
                field: "TX_TYPE",
                value: code.to_string(),
            }),
        }
    }
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
#[repr(u8)]
pub enum TxStatus {
    /// Транзакция успешно выполнена.
    Success = 0,
    /// Транзакция завершилась ошибкой.
    Failure = 1,
    /// Транзакция ожидает обработки.
    Pending = 2,
}

impl TxStatus {
    /// Возвращает числовой код статуса транзакции для бинарного формата YPBankBin.
    pub(crate) fn bin_code(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::Pending => 2,
        }
    }

    /// Преобразует числовой код бинарного формата YPBankBin в статус транзакции.
    pub(crate) fn from_bin_code(code: u8) -> Result<Self, ParserError> {
        match code {
            0 => Ok(Self::Success),
            1 => Ok(Self::Failure),
            2 => Ok(Self::Pending),
            _ => Err(ParserError::InvalidField {
                field: "STATUS",
                value: code.to_string(),
            }),
        }
    }
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
    ///
    /// Используется знаковый тип для совместимости с бинарным форматом YPBankBin.
    pub amount: i64,
    /// Unix timestamp транзакции в миллисекундах.
    pub timestamp: u64,
    /// Статус обработки транзакции.
    pub status: TxStatus,
    /// Текстовое описание транзакции.
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tx_type_returns_binary_codes() {
        assert_eq!(TxType::Deposit.bin_code(), 0);
        assert_eq!(TxType::Transfer.bin_code(), 1);
        assert_eq!(TxType::Withdrawal.bin_code(), 2);
    }

    #[test]
    fn tx_type_parses_binary_codes() -> Result<(), ParserError> {
        assert_eq!(TxType::from_bin_code(0)?, TxType::Deposit);
        assert_eq!(TxType::from_bin_code(1)?, TxType::Transfer);
        assert_eq!(TxType::from_bin_code(2)?, TxType::Withdrawal);

        Ok(())
    }

    #[test]
    fn tx_type_rejects_unknown_binary_code() {
        let result = TxType::from_bin_code(3);

        assert!(matches!(
            result,
            Err(ParserError::InvalidField {
                field: "TX_TYPE",
                ..
            })
        ));
    }

    #[test]
    fn tx_status_returns_binary_codes() {
        assert_eq!(TxStatus::Success.bin_code(), 0);
        assert_eq!(TxStatus::Failure.bin_code(), 1);
        assert_eq!(TxStatus::Pending.bin_code(), 2);
    }

    #[test]
    fn tx_status_parses_binary_codes() -> Result<(), ParserError> {
        assert_eq!(TxStatus::from_bin_code(0)?, TxStatus::Success);
        assert_eq!(TxStatus::from_bin_code(1)?, TxStatus::Failure);
        assert_eq!(TxStatus::from_bin_code(2)?, TxStatus::Pending);

        Ok(())
    }

    #[test]
    fn tx_status_rejects_unknown_binary_code() {
        let result = TxStatus::from_bin_code(3);

        assert!(matches!(
            result,
            Err(ParserError::InvalidField {
                field: "STATUS",
                ..
            })
        ));
    }
}
