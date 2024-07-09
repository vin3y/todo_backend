use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOptions,
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::model::todomodel::Todo;

pub async fn get_all_todos(
    collection: &Collection<Todo>,
) -> Result<Vec<Todo>, mongodb::error::Error> {
    let mut todos = Vec::new();
    let mut cursor = collection.find(doc! {}, None).await?;
    while let Some(to_result) = cursor.try_next().await? {
        todos.push(to_result);
    }
    Ok(todos)
}
