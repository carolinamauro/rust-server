use super::command::Command;
use teloxide::{prelude::*, utils::command::BotCommands};

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        // Command::Start => {
        //     bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        // }
        Command::Search(movie) => {
            bot.send_message(msg.chat.id, format!("Buscando la película {}...", movie))
                .await?
        }
        _ => todo!(),
    };

    Ok(())
}

pub async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}
