use config::{Config, Environment, File};
use serde::Deserialize;

use rest_rs::error::Result;

#[derive(Debug, Deserialize, Default)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    pub url: String,
    pub log_level: String,
    pub database: Database,
}

impl Settings {
    pub fn new(location: &str, env_prefix: &str) -> Result<Settings> {
        let s = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("_")
                    .prefix_separator("_"),
            )
            .build()?;

        let settings = s.try_deserialize()?;

        Ok(settings)
    }
}
