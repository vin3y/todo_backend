mod controllers;
mod db;
mod model;

use crate::controllers::add_todo::add_todo;
use crate::controllers::delete_todo::delete_todo;
use crate::controllers::list_todo::get_all_todos;
use crate::controllers::todo_done::update_done;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use controllers::edit_todo::edit_todo_controller;
use db::{create_db_client, AppState};
use model::todomodel::Todo;

#[get("/")]
async fn home_function() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

#[get("/list-all")]
async fn list_function(data: web::Data<AppState>) -> impl Responder {
    let collection = data.db.collection::<Todo>("todos");
    match get_all_todos(&collection).await {
        Ok(todos) => HttpResponse::Ok().json(serde_json::json!(
            {
            "status" : "success",
            "statusCode" : 200,
            "data" :todos
            }
        )),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status" : "failed",
            "statusCode" : 401,
            "message" : format!("An error occured: {}", e)
        })),
    }
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
            .service(list_function)
            .service(add_todo)
            .service(update_done)
            .route("/delete-todo", web::delete().to(delete_todo))
            .route("/update-todo", web::patch().to(edit_todo_controller))
            .default_service(web::route().to(not_found_route_code))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
