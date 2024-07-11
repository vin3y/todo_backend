use crate::{db::AppState, model::todomodel::Todo};
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::Collection;
use serde::Deserialize;
use serde_json::{json, Deserializer, Serializer};

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[post("/add-todo")]
async fn add_todo(data: web::Data<AppState>, new_todo: web::Json<CreateTodo>) -> impl Responder {
    let collection = data.db.collection::<Todo>("todos");
    let todo = Todo::new(new_todo.title.clone());

    match collection.insert_one(todo, None).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "statusCode": 200,
            "message": "Todo added successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "failed",
            "statusCode": 500,
            "message": format!("An error occurred: {}", e)
        })),
    }
}
