use crate::types::transaction::NewTransaction;
use crate::types::transaction_type::TransactionType;
use crate::types::ledger::Ledger;

use super::data_handler::ledger_operations::get_ledger;
use super::data_handler::ledger_operations::insert_ledger;
use super::data_handler::ledger_operations::update_ledger;
use super::data_handler::user_operations::get_user_by_user_id;
use super::data_handler::transaction_operations::insert_transaction;
use super::ledger_handler::create_ledger_from_transaction;
use super::text_helper::generate_transaction_response;

pub fn execute_transaction(transaction: NewTransaction) -> String {
    if transaction.initiator == transaction.reciever {
        return "".to_string();
    }
    // fetch reciever data
    let reciever = match get_user_by_user_id(&transaction.reciever) {
        Ok(mut user) => user.pop().unwrap(),
        Err(e) => return e.to_string(), 
    };
    // fetch sender data
    let sender = match get_user_by_user_id(&transaction.initiator) {
        Ok(mut user) => user.pop().unwrap(),
        Err(e) => return e.to_string(), 
    };
    // save transaction data before giving away ownership
    let sum = transaction.sum;
    let tr_type = match &transaction.transaction_type {
        TransactionType::Loan => TransactionType::Loan,
        TransactionType::Payment => TransactionType::Payment,
    };
    // execute payment/loan
    let succ = match transaction.transaction_type {
        TransactionType::Loan => execute_loan(transaction),
        TransactionType::Payment => execute_payment(transaction),
    };
    // return response based on success of transaction
    generate_transaction_response(sum, sender, reciever, succ, tr_type)
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
    if transaction.reciever == transaction.initiator {
        return false;
    }
    let ledger_option: Option<Ledger> = match get_ledger(&transaction.reciever, &transaction.initiator) {
        Ok(mut ledgers) => {
            // query could be Ok() but empty, since
            // the ledger might not yet exist
            if !ledgers.is_empty() {
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
            Ok(_) => insert_transaction(transaction).is_ok(),
            Err(_) => false,
        }
    } else {
        false
    }
}