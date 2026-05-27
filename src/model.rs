#[derive(Debug, PartialEq, Eq)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}
#[derive(Debug, PartialEq, Eq)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
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
