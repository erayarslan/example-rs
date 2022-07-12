use crate::comment;
use crate::search;

pub struct ServiceContainer {
    pub comment_service: comment::service::CommentService,
    pub search_service: search::service::SearchService,
}

impl ServiceContainer {
    pub fn new(comment_service: comment::service::CommentService,
               search_service: search::service::SearchService) -> Self {
        ServiceContainer {
            comment_service,
            search_service,
        }
    }
}

pub struct AppState {
    pub service_container: ServiceContainer,
}