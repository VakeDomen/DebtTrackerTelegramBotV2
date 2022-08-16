use teloxide::types::UserId;

use crate::types::{user::User, transaction_type::TransactionType, ledger::Ledger};

pub const LOAN_DESCRIPTION: &str = "/loan <amount> <@people>\nThe command loans money to the specified people. The action is constructed from 3 parts (<command> <amount> <poeple>). Amount is a numeric value where the decimal point may be specified with y dot '.' and not comma ','. You may list as many people as you wish as long as they are tagged with a mention (@name). The fee will bi equaly split among the target people.";
pub const PAY_DESCRIPTION: &str = "/pay <amount> <@people>\nThe command will repay the full amount specified to all mentioned people (as long as they are mentioned with @name). If you pay more than you own, the reviever will own you the difference after the transaction completes.";
pub const HISTORY_DESCRIPTION: &str = "/history <number of transactions>\nThe histroy command will display the last completed transactions. You may also specify the amount of transactions displayed, but it defaults to 10 if the argument is not specified.";
pub const BALANCE_DESCRIPTION: &str = "/balance\nThe command will display the current state of debt.";

pub fn generate_transaction_response(
    sum: i32, 
    sender: User, 
    reciever: User, 
    success: bool, 
    trans_type: TransactionType
) -> String {
    let action = match trans_type {
        TransactionType::Loan => "loaned".to_string(),
        TransactionType::Payment => "payed".to_string(),
    };
    match success {
        true => format!(
            "{} {} {}€ to {}!", 
            sender.username,
            action,
            (sum as f32 / 100.), 
            reciever.username
        ),
       false => "Oops! Something went wrong when processing the transaction! :(".to_string()
    }
}

pub fn generate_balance_response(
    ledgers: Vec<Ledger>,
    users: Vec<User>,
) -> String {
    let mut out = "".to_string();
    for ledger in ledgers.into_iter() {
        if ledger.sum == 0 {
            continue;
        }
        let borrower = match map_user_id_to_username(&ledger.borrower, &users) {
            Some(name) => name,
            None => continue
        };
        let owes = match map_user_id_to_username(&ledger.owes, &users) {
            Some(name) => name,
            None => continue
        };
        let sum = ledger.sum as f32 / 100.;
        out = format!("{}\n{} owes {}€ to {}", out, borrower, sum, owes);
    }
    out
}

fn map_user_id_to_username(uid: &UserId, users: &Vec<User>) -> Option<String> {
    for user in users.into_iter() {
        if user.user_id == *uid {
            return Some(user.username.clone())
        }
    }
    None
}