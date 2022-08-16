use crate::types::{ledger::NewLedger, transaction::NewTransaction};

pub fn create_ledger_from_transaction(transaction: &NewTransaction) -> NewLedger {
    NewLedger {
        borrower: serde_json::to_string(&transaction.reciever).unwrap(),
        owes: serde_json::to_string(&transaction.initiator).unwrap(),
        sum: 0
    }
}