use crate::{comment, kafka};
use crate::search;

pub struct ServiceContainer {
    pub comment_service: comment::service::CommentService,
    pub search_service: search::service::SearchService,
    pub kafka_service: kafka::service::KafkaService,
}

impl ServiceContainer {
    pub fn new(comment_service: comment::service::CommentService,
               search_service: search::service::SearchService,
               kafka_service: kafka::service::KafkaService) -> Self {
        ServiceContainer {
            comment_service,
            search_service,
            kafka_service,
        }
    }
}

pub struct AppState {
    pub service_container: ServiceContainer,
}