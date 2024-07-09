use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn home_function() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(home_function)))
        .bind("localhost:8080")?
        .run()
        .await
}
