use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub tmdb_api_key: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Deserialize)]
pub struct Endpoint {
    pub url: String,
    pub region: String,
    pub bucket_name: String,
    #[serde(default = "default_path")]
    pub path: String,
}

fn default_path() -> String {
    "".to_string()
}

impl Config {
    pub fn load(path: &str) -> Result<Config, ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .and_then(|c| c.try_deserialize::<Config>())
    }
}
