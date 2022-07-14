mod json_error;
mod comment;
mod search;
mod settings;
mod app_state;
mod utils;
mod status;
mod kafka;

use std::{env, thread};
use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;
use mongodb::{options::ClientOptions, Client, Database};
use mongodb::bson::doc;
use ntex::web;
use ntex::web::{middleware, App, server, resource, types::State};
use rdkafka::ClientConfig;
use tokio::runtime::Runtime;

fn kafka() -> ClientConfig {
    ClientConfig::new()
        .set("bootstrap.servers", &settings::SETTINGS.kafka.servers)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "SCRAM-SHA-512")
        .set("sasl.username", &settings::SETTINGS.kafka.username)
        .set("sasl.password", &settings::SETTINGS.kafka.password)
        .clone()
}

async fn elasticsearch() -> Result<Elasticsearch, elasticsearch::Error> {
    let transport = Transport::single_node(&settings::SETTINGS.elastic_uri)?;
    let client = Elasticsearch::new(transport);

    let ping = client.ping();
    let ping_result = ping.send().await;

    match ping_result {
        Ok(_) => Ok(client),
        Err(e) => panic!("{}", e.to_string())
    }
}

async fn mongodb() -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(&settings::SETTINGS.mongo_uri).await?;
    client_options.app_name = Some(settings::SETTINGS.app_name.to_owned());

    let client = Client::with_options(client_options)?;

    let response_result = client
        .database("admin")
        .run_command(doc! {"ping": 1}, None).await;

    match response_result {
        Ok(_) => Ok(client.database(&settings::SETTINGS.database)),
        Err(e) => panic!("{}", e.to_string())
    }
}

async fn get_app_state() -> State<app_state::AppState> {
    let mongo = mongodb().await.unwrap();
    let elastic = elasticsearch().await.unwrap();
    let kafka_config = kafka();

    let comment_repository = comment::repository::CommentRepository::new(mongo);
    let comment_service = comment::service::CommentService::new(comment_repository);

    let search_repository = search::repository::SearchRepository::new(elastic);
    let search_service = search::service::SearchService::new(search_repository);

    let kafka_service = kafka::service::KafkaService::new(&kafka_config);

    let runtime = Runtime::new().unwrap();

    let _ = thread::spawn(move || {
        runtime.block_on(async move {
            kafka::service::KafkaService::new(&kafka_config)
                .consume(&settings::SETTINGS.kafka.topic)
                .await
        })
    });

    let service_container = app_state::ServiceContainer::new(
        comment_service,
        search_service,
        kafka_service,
    );

    State::new(app_state::AppState { service_container })
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "ntex=info,librdkafka=trace,rdkafka::client=debug");
    env_logger::init();

    let app_state = get_app_state().await;

    server(move || {
        App::new()
            .app_state(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(utils::auth_middleware::Auth::default())
            .service((
                resource("/comments").route(web::get().to(comment::controller::get)),
                resource("/comments/{id}").route(web::get().to(comment::controller::get_by_id)),
                resource("/search").route(web::get().to(search::controller::search)),
                resource("/status").route(web::get().to(status::controller::status)),
            ))
    })
        .bind(format!("0.0.0.0:{}", settings::SETTINGS.port))?
        .run()
        .await
}