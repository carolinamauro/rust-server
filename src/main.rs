use dotenv::dotenv;
use grupo_gpt::{bot::init::run_bot};

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();
    run_bot().await;
}
