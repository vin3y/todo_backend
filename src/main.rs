use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

#[get("/")]
async fn home_function() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // println!("{:?}", env::var("MONGO_DB_URI"));

    let mongo_db_connection: String = env::var("MONGO_DB_URI").expect("No env module found");
    let mut client_options: ClientOptions =
        ClientOptions::parse(&mongo_db_connection).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    match client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
    {
        Ok(_) => println!("Connected successfully to MongoDB!"),
        Err(e) => eprintln!("Failed to connect to MongoDB: {}", e),
    }

    // println!("Connected successfully to mongodb!");

    HttpServer::new(|| App::new().service(home_function))
        .bind("localhost:8080")?
        .run()
        .await
}
