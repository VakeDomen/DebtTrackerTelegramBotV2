

use std::error::Error;

use teloxide::types::{Message, MessageKind, MediaKind};
use crate::types::transaction::NewTransaction;

pub fn validate_loan_message(message: Message) -> Result<Vec<NewTransaction>, Box<dyn Error>> {
    println!("{:#?}", message);
    match message.kind {
        MessageKind::Common(mes) => {
            match mes.media_kind {
                MediaKind::Text(media) => { 
                    let amount = match extract_loan_amount(&media.text) {
                        Some(a) => a,
                        None => return Err("Amount not specified correctly".into())
                    };
                    
                    Ok(vec![])
                 },
                _ => Err("Not text media".into())
            }
        },
        _ => Err("Not common message".into())
    }
}

pub fn validate_pay_message(message: Message) -> Result<Vec<NewTransaction>, Box<dyn Error>> {
    Ok(vec![])
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