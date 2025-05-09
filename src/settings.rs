use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub mail: EmailConfig,
}

impl AppSettings {
    pub fn new(loc: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let c = Config::builder().add_source(File::with_name(loc)).build()?;

        let settings = c.try_deserialize()?;

        Ok(settings)
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct EmailConfig {
    pub driver: Option<String>,
    pub host: String,
    pub port: Option<i32>,
    pub username: String,
    pub password: String,
    pub encryption: Option<String>,
    pub from_address: String,
    pub from_name: String,
}
