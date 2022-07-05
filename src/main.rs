use teloxide::{prelude::*, utils::command::BotCommands};
use std::error::Error;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Starts the bot.")]
    Start
    
}


async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?;
        }
       
    }
    Ok(())
}
