

use std::io::Error;
use teloxide::types::Message;
use crate::types::transaction::NewTransaction;

pub fn validate_loan_message(message: Message) -> Result<Vec<NewTransaction>, Error> {
    Ok(vec![])
}

pub fn validate_pay_message(message: Message) -> Result<Vec<NewTransaction>, Error> {
    Ok(vec![])
}
