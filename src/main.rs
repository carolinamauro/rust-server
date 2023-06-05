use dotenv::dotenv;
use grupo_gpt::{bot::init::run_bot, db::mongo::start_db_connection, cine::cine::Cine};

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();
    let _cine = Cine::new();

    start_db_connection().await;
    run_bot().await;
}
