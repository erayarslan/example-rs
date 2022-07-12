use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};
use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub msg: String,
    pub status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl WebResponseError for Error {
    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        let err_json = json!({ "error": self.msg, "status": self.status });
        let status_code = StatusCode::from_u16(self.status).unwrap();
        HttpResponse::build(status_code).json(&err_json)
    }
}