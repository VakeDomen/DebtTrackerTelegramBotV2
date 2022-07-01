use log::info;
use teloxide::{prelude::*, utils::command::BotCommands, types::MessageKind, types::MediaKind};
use std::error::Error;
use std::{env};
use dotenv::dotenv;

mod helpers;
mod types;
use crate::types::transaction::Transaction;
use crate::types::ledger::Ledger;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


#[tokio::main]
async fn main() {
    // setup env variables
    dotenv().ok();
    env::set_var("TELOXIDE_TOKEN", env::var("TELOXIDE_TOKEN").expect("$TELOXIDE_TOKEN is not set"));
    // env::set_var("CHAT_ID", env::var("CHAT_ID").expect("$CHAT_ID is not set"));
    
    // init stuff
    pretty_env_logger::init();
    // init_queue();

    // run bot
    let bot = Bot::from_env().auto_send();
    info!("Running telegram bot!");
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Claim the task as done. This will award you the task points.")]
    Claim(String),
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help                   => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Claim(item_id)     => { bot.send_message(message.chat.id, claim_task(&bot, message, item_id)).await? },
    };
    Ok(())
}

fn claim_task(
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
