use teloxide::types::UserId;

use crate::types::{ledger::Ledger};

pub fn find_ledgers_by_loaners_and_borrowers(loaners: &Vec<UserId>, borrowers: &Vec<UserId>) -> Vec<Ledger> {
    vec![]
}