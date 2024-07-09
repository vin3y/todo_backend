use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home_function() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home_function))
        .bind("localhost:8080")?
        .run()
        .await
}
