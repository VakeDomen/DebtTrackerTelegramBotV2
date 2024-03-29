use serde::{Serialize, Deserialize};
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize, derive_more::Display, EnumString, Clone)]
pub enum TransactionType {
    Loan,
    Payment,
}