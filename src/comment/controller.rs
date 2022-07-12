use ntex::http::StatusCode;
use ntex::web;
use ntex::web::{HttpRequest, HttpResponse, types::State, types::Query};
use crate::app_state;
use crate::comment::model::CommentRequest;
use crate::json_error::Error;

pub async fn get(app_state: State<app_state::AppState>, _req: HttpRequest, query: Query<CommentRequest>) -> Result<HttpResponse, Error> {
    let size = query.size.unwrap_or(20i64);
    let name = query.name.as_ref();

    let result = app_state.service_container.comment_service.get(name, size).await;

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(&data)),
        Err(e) => Err(Error {
            msg: e,
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        })
    }
}

pub async fn get_by_id(app_state: State<app_state::AppState>, _req: HttpRequest, path: web::types::Path<(String, )>) -> Result<HttpResponse, Error> {
    let (id, ) = path.into_inner();
    let result = app_state.service_container.comment_service.get_by_id(&id).await;

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(&data)),
        Err(e) => Err(Error {
            msg: e,
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        })
    }
}