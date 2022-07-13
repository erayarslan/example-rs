use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, types::State, types::Query};
use crate::app_state;
use crate::json_error::Error;
use crate::search::model::SearchRequest;

pub async fn search(app_state: State<app_state::AppState>, _req: HttpRequest, query: Query<SearchRequest>) -> Result<HttpResponse, Error> {
    let q = query.q.as_ref().map(|x| x.as_str());

    let result = app_state.service_container.search_service.search(q).await;

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(&data)),
        Err(e) => Err(Error {
            msg: e.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        })
    }
}