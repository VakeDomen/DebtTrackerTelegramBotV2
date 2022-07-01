use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, derive_more::Display)]
pub enum TransactionType {
    Loan,
    Payment,
}