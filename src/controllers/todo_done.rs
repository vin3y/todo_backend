use crate::{db::AppState, model::todomodel::Todo};
use actix_web::{
    patch,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: String,
}

#[patch("/todo-done")]
async fn update_done(data: Data<AppState>, update_data: Json<UpdateTodo>) -> impl Responder {
    let collection: Collection<Todo> = data.db.collection::<Todo>("todos");
    let object_id = match ObjectId::parse_str(&update_data.id) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "failed",
                "statusCode": 400,
                "message": "Invalid Object ID"
            }));
        }
    };

    // First, fetch the current todo item
    let filter = doc! {"_id": object_id};
    match collection.find_one(filter.clone(), None).await {
        Ok(Some(todo)) => {
            // Toggle the done status
            let new_status = !todo.done;
            let update = doc! {"$set": {"done": new_status}};

            match collection.update_one(filter, update, None).await {
                Ok(result) => {
                    if result.modified_count == 1 {
                        HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "statusCode": 200,
                            "message": "Todo status updated successfully",
                            "newStatus": new_status
                        }))
                    } else {
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "failed",
                            "statusCode": 500,
                            "message": "Failed to update todo status"
                        }))
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "statusCode": 500,
                    "message": format!("Database error: {}", e)
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "failed",
            "statusCode": 404,
            "message": "Todo not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "statusCode": 500,
            "message": format!("Database error: {}", e)
        })),
    }
}
