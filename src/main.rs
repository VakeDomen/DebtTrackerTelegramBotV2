use log::info;
use teloxide::{prelude::*, utils::command::BotCommands, types::MessageKind, types::MediaKind};
use std::error::Error;
use std::env;
use dotenv::dotenv;

mod helpers;
mod types;
use helpers::message_validator;
use helpers::text_helper::PAY_DESCRIPTION;

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
    };
    Ok(())
}


fn loan(
    _: &AutoSend<Bot>,
    message: Message,
    task_index: String
) -> String {
    info!("Some user is claiming a task!");
    match message_validator::validate_loan_message(message) {
        Ok(transactions) => execute_transactions(transactions).join("\n"),
        Err(e) => e
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
