use crate::types::transaction::Transaction;
use crate::types::transaction_type::TransactionType;

pub fn execute_transaction(transaction: Transaction) {
    match transaction.transaction_type {
        TransactionType::Loan => execute_loan(transaction),
        TransactionType::Payment => execute_payment(transaction),
    }
}

pub fn execute_transactions(mut transactions: Vec<Transaction>) {
    let num_of_transactions = transactions.len();
    for _ in 0..num_of_transactions {
        if let Some(transaction) = transactions.pop() {
            execute_transaction(transaction);
        }
    }
}

fn execute_loan(transaction: Transaction) {
    
}

fn execute_payment(transaction: Transaction) {

}