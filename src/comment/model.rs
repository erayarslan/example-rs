use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize)]
pub struct Comment {
    _id: ObjectId,
    name: String,
    email: String,
    movie_id: ObjectId,
    text: String,
    date: DateTime,
}

#[derive(Serialize)]
pub struct CommentResponse {
    _id: String,
    name: String,
    email: String,
    movie_id: String,
    text: String,
    date: String,
}

#[derive(Deserialize)]
pub struct CommentRequest {
    pub size: Option<i64>,
    pub name: Option<String>,
}

impl CommentResponse {
    pub fn new(comment: Comment) -> CommentResponse {
        CommentResponse {
            _id: comment._id.to_string(),
            name: comment.name,
            email: comment.email,
            movie_id: comment.movie_id.to_string(),
            text: comment.text,
            date: comment.date.to_string(),
        }
    }
}