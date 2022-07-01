use std::str::FromStr;

use chrono::NaiveDateTime;
use teloxide::types::UserId;
use super::{transaction_type::TransactionType};
use uuid::Uuid;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub initiator: UserId,
    pub reciever: UserId,
    pub sum: i32,
    pub description: String,
    pub created: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct SqliteTransaction {
    pub id: String,
    pub transaction_type: String,
    pub initiator: String,
    pub reciever: String,
    pub sum: i32,
    pub description: String,
    pub created: String,
}

pub struct NewTransaction {
    pub transaction_type: TransactionType,
    pub initiator: UserId,
    pub reciever: UserId,
    pub sum: i32,
    pub description: String,
    pub created: NaiveDateTime,
}

impl From<SqliteTransaction> for Transaction {
    fn from(transaction: SqliteTransaction) -> Self {
        Self { 
            id: transaction.id, 
            transaction_type: TransactionType::from(transaction.transaction_type), 
            initiator: serde_json::from_str(&transaction.initiator).unwrap(), 
            reciever: serde_json::from_str(&transaction.reciever).unwrap(), 
            sum: transaction.sum, 
            description: transaction.description, 
            created: NaiveDateTime::from_str(&transaction.created).unwrap() 
        }
    }
}

impl From<Transaction> for SqliteTransaction {
    fn from(transaction: Transaction) -> Self {
        Self { 
            id: transaction.id, 
            transaction_type: transaction.transaction_type.to_string(), 
            initiator: transaction.initiator.to_string(), 
            reciever: transaction.reciever.to_string(), 
            sum: transaction.sum, 
            description: transaction.description, 
            created: transaction.created.to_string() 
        }
    }
}

impl From<NewTransaction> for SqliteTransaction {
    fn from(transaction: NewTransaction) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(), 
            transaction_type: transaction.transaction_type.to_string(), 
            initiator: transaction.initiator.to_string(), 
            reciever: transaction.reciever.to_string(), 
            sum: transaction.sum, 
            description: transaction.description, 
            created: transaction.created.to_string() 
        }
    }
}
