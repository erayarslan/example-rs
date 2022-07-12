use futures::TryStreamExt;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use mongodb::bson::oid;
use mongodb::options::FindOptions;
use std::str::FromStr;
use crate::comment::model::Comment;

#[derive(Clone)]
pub struct CommentRepository {
    collection: Collection<Comment>,
}

impl CommentRepository {
    pub fn new(database: Database) -> CommentRepository {
        CommentRepository {
            collection: database.collection(&crate::settings::SETTINGS.collection)
        }
    }

    pub async fn get(&self, name: Option<&String>, size: i64) -> Result<Vec<Comment>, String> {
        let find_options = FindOptions::builder().limit(size).build();
        let query = name.map(|n| doc! {"name": n}).unwrap_or(doc! {});
        let result = self.collection.find(query, find_options).await;

        match result {
            Ok(cursor) => Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(e) => Err(e.to_string())
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Comment, String> {
        let object_id_result = oid::ObjectId::from_str(id);

        match object_id_result {
            Ok(object_id) => {
                let result = self.collection.find_one(doc! {"_id": object_id}, None).await;
                match result {
                    Ok(cursor) => cursor.ok_or(String::from("not found")),
                    Err(e) => Err(e.to_string())
                }
            }
            Err(e) => Err(e.to_string())
        }
    }
}