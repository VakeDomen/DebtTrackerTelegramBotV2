use chrono::NaiveDateTime;
use super::{app_user::AppUser, transaction_type::TransactionType};

pub struct Transaction {
    pub transaction_type: TransactionType,
    pub initiator: AppUser,
    pub reciever: AppUser,
    pub sum: i64,
    pub description: String,
    pub created: NaiveDateTime,
}