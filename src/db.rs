use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;

pub struct AppState {
    pub db: Database,
}

pub async fn create_db_client() -> AppState {
    dotenv().ok();

    let mongo_db_uri = env::var("MONGO_DB_URI").expect("Mongo db url expected");
    let mongo_db_db = env::var("MONGO_DB_DATABASE").expect("Couldnt find a databse");
    let client_options = ClientOptions::parse(&mongo_db_uri).await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();
    let db: Database = client.database(&mongo_db_db);
    match client
        .database(&mongo_db_db)
        .run_command(doc! {"ping" :1}, None)
        .await
    {
        Ok(_) => println!("Connected successfully to MongoDB!"),
        Err(e) => eprintln!("Failed to connect to MongoDB: {}", e),
    }

    AppState { db }
}
