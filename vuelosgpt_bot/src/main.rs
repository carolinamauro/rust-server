use teloxide::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let key = "TELOXIDE_TOKEN";
    env::set_var(key, "6273242308:AAFYv0YM10dOwLfA0UxqHUw1eG4KsTenLA0");
    
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}