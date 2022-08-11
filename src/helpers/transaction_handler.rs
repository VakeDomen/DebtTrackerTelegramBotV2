use diesel::IntoSql;

use crate::types::transaction::NewTransaction;
use crate::types::transaction_type::TransactionType;
use crate::types::ledger::Ledger;

use super::data_handler::get_ledger;
use super::data_handler::get_user_by_user_id;
use super::data_handler::insert_ledger;
use super::data_handler::insert_transaction;
use super::data_handler::update_ledger;
use super::ledger_handler::create_ledger_from_transaction;
use super::text_helper::generate_transaction_response;

pub fn execute_transaction(transaction: NewTransaction) -> String {
    let reciever = match get_user_by_user_id(&transaction.reciever) {
        Ok(mut user) => user.pop().unwrap(),
        Err(e) => return e.to_string(), 
    };
    let sender = match get_user_by_user_id(&transaction.initiator) {
        Ok(mut user) => user.pop().unwrap(),
        Err(e) => return e.to_string(), 
    };
    let sum = transaction.sum;
    let succ = match transaction.transaction_type {
        TransactionType::Loan => execute_loan(transaction),
        TransactionType::Payment => execute_payment(transaction),
    };
    generate_transaction_response(sum, sender, reciever, succ)
}

pub fn execute_transactions(mut transactions: Vec<NewTransaction>) -> Vec<String> {
    let num_of_transactions = transactions.len();
    let mut transaction_responses = vec![]; 
    for _ in 0..num_of_transactions {
        if let Some(transaction) = transactions.pop() {
            transaction_responses.push(execute_transaction(transaction));
        }
    }
    transaction_responses
}

fn execute_payment(transaction: NewTransaction) -> bool {
    // payment does the same thing as loan in terms of money flow (but users
    // find it more appealing to use when returning the money)
    execute_loan(transaction)
}

fn execute_loan(transaction: NewTransaction) -> bool {
    
    let ledger_option: Option<Ledger> = match get_ledger(&transaction.reciever, &transaction.initiator) {
        Ok(mut ledgers) => {
            // query could be Ok() but empty, since
            // the ledger might not yet exist
            if ledgers.len() > 0 {
                // if the ledger exists -> return it
                Some(ledgers.remove(0))
            } else {
                // if no ledger yet, insert an empty one
                let new_ledger = create_ledger_from_transaction(&transaction);
                match insert_ledger(new_ledger) {
                    Ok(ledger) => Some(ledger),
                    Err(_) => None
                }
            }
        },
        Err(_) => None
    };
    // if we had no errors in fetching the ledger
    if let Some(mut ledger) = ledger_option {
        ledger.sum += transaction.sum;
        match update_ledger(ledger) {
            Ok(_) => {
                match insert_transaction(transaction) { _ => () }
                true
            },
            Err(_) => false,
        }
    } else {
        false
    }
}