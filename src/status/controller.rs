use ntex::web::HttpResponse;
use crate::json_error::Error;
use crate::status::model::Status;

pub async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&Status {
        message: String::from("always true"),
    }))
}