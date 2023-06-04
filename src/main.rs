use dotenv::dotenv;
use grupo_gpt::{bot::init::run_bot, db::mongo::start_db_connection};

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();

    start_db_connection().await;
    run_bot().await;
}
