use std::{task::Context, task::Poll};
use std::borrow::Cow;
use std::future::Future;
use std::pin::Pin;
use futures::future::err;
use std::rc::Rc;
use futures::future::Either;
use ntex::service::{Service, Transform};
use ntex::web::{Error, ErrorRenderer, HttpResponse, WebRequest, WebResponse};
use serde_json::json;
use ntex::http::StatusCode;
use ntex::web::types::State;
use crate::settings;
use crate::app_state::AppState;

pub struct Auth;

impl Default for Auth {
    fn default() -> Self {
        Auth {}
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S> Transform<S> for Auth {
    type Service = AuthMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Service {
        AuthMiddleware {
            service: Rc::new(service),
        }
    }
}

impl<S, Err> Service<WebRequest<Err>> for AuthMiddleware<S>
    where
        S: Service<WebRequest<Err>, Response=WebResponse, Error=Error> + 'static,
        Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: WebRequest<Err>) -> Self::Future {
        let svc = self.service.clone();
        let auth_header_name = &settings::SETTINGS.auth_header_name;

        Box::pin(async move {
            let auth_header_value: &str = req.headers().get(auth_header_name)
                .map(|value| value.to_str().unwrap_or(""))
                .unwrap_or("");

            let is_valid_with_config = auth_header_value.eq(&settings::SETTINGS.auth_header_value);

            let app_state: Option<&State<AppState>> = req.app_state();

            let is_valid_with_mongo = app_state
                .map_or(
                    Either::Right(err(Cow::from("app_state not found"))),
                    |app_state| {
                        Either::Left(
                            app_state
                                .service_container
                                .comment_service
                                .is_exist_by_name(auth_header_value)
                        )
                    },
                )
                .await
                .unwrap_or(false);

            let is_auth_valid = is_valid_with_config || is_valid_with_mongo;

            let payload = format!("is_auth_valid={}", is_auth_valid);

            let is_produced = app_state
                .map_or(
                    Either::Right(err(Cow::from("app_state not found"))),
                    |app_state| {
                        Either::Left(
                            app_state
                                .service_container
                                .kafka_service
                                .produce(req.path(), payload.as_str())
                        )
                    },
                )
                .await
                .unwrap_or(false);

            if is_auth_valid && is_produced {
                let res = svc.call(req).await?;
                Ok(res)
            } else {
                let err_message = format!("{} not correct!", auth_header_name);

                let response = HttpResponse::Unauthorized().json(&json!({
                    "error": err_message,
                    "status": StatusCode::UNAUTHORIZED.as_u16()
                }));

                let res = req.into_response(response);
                Ok(res)
            }
        })
    }
}