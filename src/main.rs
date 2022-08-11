use log::info;
use teloxide::{prelude::*, utils::command::BotCommands, types::MessageKind, types::MediaKind};
use std::error::Error;
use std::env;
use dotenv::dotenv;

mod helpers;
mod types;
use helpers::message_validator;
use helpers::text_helper::PAY_DESCRIPTION;

use crate::helpers::data_handler::{insert_user, get_user_by_user_id, update_user};
use crate::helpers::transaction_handler::execute_transactions;

extern crate pretty_env_logger;

#[macro_use] extern crate log;
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
    Loan(String),
    #[command(description = "Pay money back to a person")]
    Pay(String),
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
        Command::Help                   => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Loan(input)     => { bot.send_message(message.chat.id, loan(&bot, message, input)).await? },
        Command::Pay(input)     => { bot.send_message(message.chat.id, pay(&bot, message, input)).await? },
        Command::Balance     => { bot.send_message(message.chat.id, balance(&bot, message)).await? },
        Command::History                   => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Stats                   => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Register                   => { bot.send_message(message.chat.id, register(&bot,message)).await? },
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
        match insert_user(types::user::NewUser::from(user)) {
            Err(e) => e.to_string(),
            Ok(created_user) => format!("Registered user as: {:?}", created_user.username)
        }

    // too many users exist -> notify invalid state
    } else if users.len() > 1 {
        "Invalid number of users with same id! Please contact the developer.".to_string()

    // user already registered -> check for username change
    } else {
        let mut existinig_user = users.pop().unwrap();
        if user.username.as_ref().unwrap().ne(&existinig_user.username) {
            existinig_user.username = user.username.as_ref().unwrap().clone();
            match update_user(existinig_user) {
                Err(e) => e.to_string(),
                Ok(updated_user) => format!("Updated user as: {:?}", updated_user.username)
            }
        } else {
            "User already registered".to_string()
        }
    }
}

fn loan(
    _: &AutoSend<Bot>,
    message: Message,
    task_index: String
) -> String {
    info!("Some user is claiming a task!");
    match message_validator::validate_loan_message(message) {
        Ok(transactions) => execute_transactions(transactions).join("\n"),
        Err(e) => e.to_string()
    }
}

fn pay(
    _: &AutoSend<Bot>,
    message: Message,
    task_index: String
) -> String {
    info!("Some user is claiming a task!");
    match message.kind {
        MessageKind::Common(mes) => {
            match mes.media_kind {
                MediaKind::Text(media) => { format!("{:#?}", media.entities) },
                _ => "Not text media".to_string()
            }
            
        },
        _ => "Not common message".to_string()
    }
}


fn balance(
    _: &AutoSend<Bot>,
    message: Message,
) -> String {
    info!("Some user is claiming a task!");
    match message.kind {
        MessageKind::Common(mes) => {
            match mes.media_kind {
                MediaKind::Text(media) => { format!("{:#?}", media.entities) },
                _ => "Not text media".to_string()
            }
            
        },
        _ => "Not common message".to_string()
    }
}
