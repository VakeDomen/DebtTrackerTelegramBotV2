use std::fmt;

use serde::{Serialize, Deserialize};
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize, derive_more::Display, EnumString)]
pub enum TransactionType {
    Loan,
    Payment,
}