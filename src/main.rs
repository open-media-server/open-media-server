mod config;
mod library;
mod metadata;
mod structure;

use std::fs;

use crate::config::config::Config;
use crate::structure::s3::parse_s3;
use dotenv::dotenv;
use library::library::Library;
use metadata::tmdb::set_metadata;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::load("examples/config").expect("Invalid configuration file");

    let structure = parse_s3(&config.endpoints[0])
        .await
        .expect("Failed to parse structure");

    let mut library = Library::from_structure(&config.endpoints[0], &structure);

    set_metadata(&config, &mut library);

    if let Ok(json) = serde_json::to_string_pretty(&library) {
        if fs::read_dir("./public").is_err() {
            fs::create_dir("./public").expect("Failed to create directory ./public");
        }
        fs::write("./public/lib.json", json).expect("Failed to write library configuration");
    }
}
