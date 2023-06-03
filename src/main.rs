use teloxide::prelude::*;
use dotenv::dotenv;
use grupo_gpt::db::mongo;

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();

    // run_bot();
    start_db_connection().await;
}

async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}

async fn start_db_connection() {
    let client = mongo::connect_to_db().await;
    if client.is_err() {
        println!("connection to db failed");
    }
}
