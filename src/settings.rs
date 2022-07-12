use std::env;
use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub mongo_uri: String,
    pub elastic_uri: String,
    pub database: String,
    pub app_name: String,
    pub collection: String,
    pub port: u16,
    pub search_index: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "".into());

        let s = Config::builder()
            .add_source(File::with_name("settings/default"))
            .add_source(File::with_name(&format!("settings/{}", run_mode)).required(true))
            .build()?;

        s.try_deserialize()
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        Settings::new().unwrap()
    };
}
