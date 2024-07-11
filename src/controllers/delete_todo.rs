use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use actix_web::{web, HttpResponse, Responder};

use crate::{db::AppState, model::todomodel::Todo};

#[derive(Debug, Deserialize)]
pub struct SelectedTodo {
    id: String,
}


pub async fn delete_todo(data:web::Data<AppState>, delete_req :web::Json<SelectedTodo>)->impl Responder {
    let collection = data.db.collection::<Todo>("todos");
    
    let object_id = match ObjectId::parse_str(&delete_req.id.trim()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "statusCode": 400,
            "message": "Invalid ObjectId format"
        })),
    };

    println!("Parsed ObjectId: {:?}", object_id);


    match collection.delete_one(doc! { "id": object_id }, None).await {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "statusCode": 200,
                    "message": "Todo deleted successfully"
                }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "status": "failed",
                    "statusCode": 404,
                    "message": "Todo not found"
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
