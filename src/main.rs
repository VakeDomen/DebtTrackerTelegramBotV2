use log::info;
use teloxide::{prelude::*, utils::command::BotCommands, types::MessageKind, types::MediaKind};
use std::error::Error;
use std::env;
use dotenv::dotenv;

mod helpers;
mod types;
use helpers::message_validator;

use crate::helpers::data_handler::{user_operations::{insert_user, get_user_by_user_id, update_user}, chat_operations::{insert_user_into_room, is_user_in_chat, get_chat_users}, ledger_operations::get_group_ledgers};
use crate::helpers::transaction_handler::execute_transactions;

extern crate strum;
extern crate pretty_env_logger;

#[macro_use] extern crate diesel;

#[tokio::main]
async fn main() {
    // setup env variables
    dotenv().ok();
    env::set_var("TELOXIDE_TOKEN", env::var("TELOXIDE_TOKEN").expect("$TELOXIDE_TOKEN is not set"));
    // init stuff
    pretty_env_logger::init();
    // run bot
    let bot = Bot::from_env().auto_send();
    info!("Running telegram bot!");
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display help text")]
    Help,
    #[command(description = "Loan money to (multiple) people")]
    Loan,
    #[command(description = "Pay money back to a person")]
    Pay,
    #[command(description = "Show ledger balance")]
    Balance,
    #[command(description = "Show past transactions")]
    History,
    #[command(description = "Show balance statistics")]
    Stats,
    #[command(description = "Register self to use the tracker")]
    Register,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Loan => { bot.send_message(message.chat.id, loan(&bot, message)).await? },
        Command::Pay => { bot.send_message(message.chat.id, pay(&bot, message)).await? },
        Command::Balance => { bot.send_message(message.chat.id, balance(&bot, message)).await? },
        Command::History => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Stats => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Register => { bot.send_message(message.chat.id, register(&bot,message)).await? },
    };
    Ok(())
}

fn register(
    _: &AutoSend<Bot>,
    message: Message,
) -> String {
    info!("User is signing up for the tracker!");
    // check valid teloxide message
    let user = match message.from() {
        None => return "Oops something went wrong! Can't detect user.".to_string(),
        Some(user) => user
    };
    // check if user has username setup
    match user.username {
        None => return "Please setup a telegram username (under settings -> edit profile) so I can identify you.".to_string(),
        Some(_) =>  {}
    }
    // find all redistered users with same id (should be vec of 0 or 1 users)
    let mut users = match get_user_by_user_id(&user.id) {
        Err(e) => return e.to_string(),
        Ok(users) => users
    };
    // user does not exist ->  register
    if users.is_empty() {
        let created_user = match insert_user(types::user::NewUser::from(user)) {
            Err(e) => return e.to_string(),
            Ok(created_user) => created_user
        };
        match insert_user_into_room(&created_user.user_id, &message.chat.id) {
            Err(e) => e.to_string(),
            Ok(_) => format!("Registered user as: {:?}", created_user.username)
        }
    // too many users exist -> notify invalid state
    } else if users.len() > 1 {
        "Invalid number of users with same id! Please contact the developer.".to_string()
    // user already registered -> check for username change
    // also check if registering from new chat
    } else {
        let mut resp = "User already registered".to_string();
        // check if new chat should be inserted
        match is_user_in_chat(user.id, message.chat.id) {
            Err(e) => return e.to_string(),
            Ok(b) => {
                if !b {
                    match insert_user_into_room(&user.id, &message.chat.id) {
                        Err(e) => return e.to_string(),
                        Ok(_) => resp = "You have been added to chat!".to_string()
                    } 
                }
            },
        }
        // check for username change
        let mut existinig_user = users.pop().unwrap();
        if user.username.as_ref().unwrap().ne(&existinig_user.username) {
            existinig_user.username = user.username.as_ref().unwrap().clone();
            match update_user(existinig_user) {
                Err(e) => e.to_string(),
                Ok(updated_user) => format!("Updated user as: {:?}", updated_user.username)
            }
        } else {
            resp
        }
    }
}

fn loan(
    _: &AutoSend<Bot>,
    message: Message,
) -> String {
    info!("User is executing a loan!");
    match message_validator::validate_loan_message(message) {
        Ok(transactions) => execute_transactions(transactions).join("\n"),
        Err(e) => e.to_string()
    }
}

fn pay(
    _: &AutoSend<Bot>,
    message: Message,
) -> String {
    info!("User is executing a payment!");
    match message_validator::validate_pay_message(message) {
        Ok(transactions) => execute_transactions(transactions).join("\n"),
        Err(e) => e.to_string()
    }
}


fn balance(
    _: &AutoSend<Bot>,
    message: Message,
) -> String {
    info!("Some user is checking balance!");
    let users = match get_chat_users(&message.chat.id) {
        Ok(users) => users,
        Err(e) => return e.to_string()
    };
    let ledgers = match get_group_ledgers(&users) {
        Ok(ledgers) => ledgers,
        Err(e) => return e.to_string()
    };
    format!("{:#?}", ledgers)
}

