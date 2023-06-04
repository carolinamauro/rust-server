use dotenv::dotenv;
use grupo_gpt::{db::mongo::start_db_connection, bot::setup::run_bot};

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();

    start_db_connection().await;
    run_bot().await;
}