use teloxide::types::UserId;

use crate::types::{ledger::{Ledger, NewLedger}, transaction::{Transaction, NewTransaction}};

pub fn find_ledgers_by_loaners_and_borrowers(loaners: &Vec<UserId>, borrowers: &Vec<UserId>) -> Vec<Ledger> {
    vec![]
}

pub fn create_ledger_from_transaction(transaction: &NewTransaction) -> NewLedger {
    NewLedger {
        borrower: serde_json::to_string(&transaction.reciever).unwrap(),
        owes: serde_json::to_string(&transaction.initiator).unwrap(),
        sum: transaction.sum
    }
}