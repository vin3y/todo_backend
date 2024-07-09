mod db;
mod model;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

use db::{create_db_client, AppState};

#[get("/")]
async fn home_function() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

async fn not_found_route_code() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!(
        {
            "status":"failed",
            "statusError":404,
            "message" : "The route not found"
        }
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(create_db_client().await);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(home_function)
            .default_service(web::route().to(not_found_route_code))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
