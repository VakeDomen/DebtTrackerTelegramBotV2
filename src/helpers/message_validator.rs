extern crate chrono;

use std::error::Error;
use serde::__private::de;
use teloxide::types::{Message, MessageKind, MediaKind, MediaText};
use chrono::Utc;

use crate::{types::{transaction::NewTransaction, user::User}, helpers::data_handler::get_user_by_user_id};

use super::data_handler::get_user_by_username;


pub fn validate_loan_message(message: Message) -> Result<Vec<NewTransaction>, Box<dyn Error>> {
    println!("{:#?}", message);

    // find sender -> throw any invalid states
    let user = match extract_user(&message) {
        Ok(user) => user,
        Err(e) => return Err(e.into()),
    };
    // check message type
    let mes = match &message.kind {
        MessageKind::Common(mes) => mes,
        _ => return Err("Not common message".into()),
    };
    // extract media (text)
    let media = match &mes.media_kind {
        MediaKind::Text(media) => media,
        _ => return Err("Not text media".into()),
    };
    // extract amount to be loaned to recievers
    let amount = match extract_loan_amount(&media.text) {
        Some(a) => a,
        None => return Err("Amount not specified correctly".into()),
    };
    // extract all recievers
    let recievers = match extract_recievers(media) {
        Ok(recv) => recv,
        Err(e) => return Err(e.into()),
    };
    println!("recievers {:#?}", recievers);
    // extract description
    let desctription = match extract_description(&media.text) {
        Ok(desc) => desc,
        Err(e) => return Err(e.into()),
    };
    // convert into transactions
    let transactions = match into_transactions(user, amount, recievers, desctription) {
        Ok(transactions) => transactions,
        Err(e) => return Err(e.into()),
    };
    Ok(transactions)
}

fn extract_description(text: &String) -> Result<String, Box<dyn Error>> {
    Ok(text.to_string())
}

pub fn validate_pay_message(message: Message) -> Result<Vec<NewTransaction>, Box<dyn Error>> {
    Ok(vec![])
}

fn into_transactions(sender: User, amount: f64, recievers: Vec<User>, description: String) -> Result<Vec<NewTransaction>, Box<dyn Error>> {
    if recievers.len() == 0 {
        return Err("No recievers recognised.".into());
    }
    let int_amount = (amount * 100.) as i32;
    let one_share = int_amount / recievers.len() as i32;
    let mut transactions = vec![];
    recievers.into_iter().for_each(|reciever| {
        transactions.push(NewTransaction {
            transaction_type: crate::types::transaction_type::TransactionType::Loan,
            initiator: sender.user_id,
            reciever: reciever.user_id,
            sum: one_share,
            description: description.clone(),
            created: Utc::now().naive_utc(),
        });
    });
    Ok(transactions)
}


fn extract_loan_amount(text: &String) -> Option<f64> {
    let text_fragments = text.split_whitespace().collect::<Vec<&str>>();
    if text_fragments.len() < 2 {
        return None;
    }
    let amount_fragment = text_fragments[1].replace(",", ".");
    let amount = amount_fragment.parse::<f64>();
    println!("{:#?}", amount);
    match amount {
        Ok(a) => Some(a),
        Err(_) => None,
    }
}

fn extract_recievers(message: &MediaText)  -> Result<Vec<User>, Box<dyn Error>> {
    let mut users = vec![];
    for entity in message.to_owned().entities.into_iter() {
        let username = match entity.kind {
            teloxide::types::MessageEntityKind::Mention => &message.text[(entity.offset + 1)..(entity.offset+entity.length)],
            _ => continue
        };
        let user = match get_user_by_username(username.to_owned()) {
            Ok(mut found_users) => {
                let u = match found_users.pop() {
                    Some(u) => u,
                    None => continue,
                };
                u
            },
            Err(e) => return Err(e.into())
        };
        users.push(user);
    }
    Ok(users)
}

fn extract_user(message: &Message) -> Result<User, Box<dyn Error>> {
    let user_id = match message.from() {
        None => return Err("You don't seem to be registered. Just type /register and retry the command.".into()),
        Some(user) => user.id
    };
    match get_user_by_user_id(&user_id) {
        Err(e) => Err(e.into()),
        Ok(mut users) => {
            if users.len() == 1 {
                match users.pop() {
                    None => Err("You don't seem to be registered. Just type /register and retry the command.".into()),
                    Some(u) => Ok(u)
                }
            } else {
                Err("Seems like there is an issue with too many registered users with the same ID. Please contact the developer.".into())
            }
        }
    }
}