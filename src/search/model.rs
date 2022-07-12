use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchRequest {
    pub q: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Order {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ElasticHit<T> {
    pub _source: T,
}

#[derive(Deserialize)]
pub struct ElasticHits<T> {
    pub hits: Vec<ElasticHit<T>>,
}

#[derive(Deserialize)]
pub struct ElasticResponse<T> {
    pub hits: ElasticHits<T>,
}