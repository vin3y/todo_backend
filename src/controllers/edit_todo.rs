use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::Deserialize;

use crate::{db::AppState, model::todomodel::Todo};
#[derive(Debug, Deserialize)]
pub struct EditTodoStruct {
    pub id: String,
    pub new_name: String,
}

pub async fn edit_todo_controller(
    data: web::Data<AppState>,
    editing_todo: web::Json<EditTodoStruct>,
) -> impl Responder {
    let collection = data.db.collection::<Todo>("todos");
    let object_id = match ObjectId::parse_str(&editing_todo.id) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status" : "failed",
                "statusCode" : 404,
                "message" : "Object id not compatable"
            }))
        }
    };

    let filter: Document = doc! {"_id" : object_id};
    let update: Document = doc! {"$set" : {"title" : &editing_todo.new_name.to_string()}};

    match collection.update_one(filter, update, None).await {
        Ok(update_res) => {
            if update_res.matched_count == 1 {
                HttpResponse::Ok().json(serde_json::json!({
                    "status" : "success",
                    "statusCode" : 200,
                    "message" : format!("the todo was modified")
                }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "status" : "error",
                    "statusCode" : "401",
                    "message" : format!("the todo was not found")
                }))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "failed",
            "statusCode": 500,
            "message": format!("An error occurred: {}", e)
        })),
    }
}
