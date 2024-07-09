mod db;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

use db::{create_db_client, AppState};

#[get("/")]
async fn home_function() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(create_db_client().await);
    HttpServer::new(move || App::new().app_data(state.clone()).service(home_function))
        .bind("localhost:8080")?
        .run()
        .await
}
