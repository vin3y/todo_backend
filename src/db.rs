use std::env;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub struct AppState {
    pub db: Database,
}

pub async fn create_db_client() -> AppState {
    dotenv().ok();

    let mongo_db_uri = env::var("MONGO_DB_URI").expect("Mongo db url expected");
    let client_options = ClientOptions::parse(&mongo_db_uri).await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();
    let db: Database = client.database("admin");
    match client
        .database("admin")
        .run_command(doc! {"ping" :1}, None)
        .await
    {
        Ok(_) => println!("Connected successfully to MongoDB!"),
        Err(e) => eprintln!("Failed to connect to MongoDB: {}", e),
    }

    AppState { db }
}
