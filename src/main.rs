use teloxide::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
