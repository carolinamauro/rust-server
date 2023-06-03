use mongodb::{Client, options::{ClientOptions, ServerApi, ServerApiVersion}, bson::doc};
use dotenv;

pub async fn connect_to_db() -> mongodb::error::Result<()> {
    if let Ok(key) = dotenv::var("MONGO_URL") {
        let client = create_client(&key).await?;
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
        .await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");
    }

    Ok(())
}

async fn create_client(key: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(key).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.app_name = Some("Cinema Telegram Bot".to_string());
    client_options.server_api = Some(server_api);
    Client::with_options(client_options)
}