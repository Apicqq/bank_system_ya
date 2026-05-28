use crate::errors::ParserError;
use std::fmt::{Display, Formatter};
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

impl Display for TxType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deposit => write!(f, "DEPOSIT"),
            Self::Transfer => write!(f, "TRANSFER"),
            Self::Withdrawal => write!(f, "WITHDRAWAL"),
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

impl Display for TxStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Failure => write!(f, "FAILURE"),
            Self::Pending => write!(f, "PENDING"),
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
