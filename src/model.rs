use crate::errors::ParserError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum TxType {
    Deposit,
    Transfer,
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

#[derive(Debug, PartialEq, Eq)]
pub enum TxStatus {
    Success,
    Failure,
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
#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TxStatus,
    pub description: String,
}
