mod config;
mod media_config;
mod metadata;
mod structure;

use crate::config::config::Config;
use crate::structure::s3::parse_s3;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::load("examples/config").expect("Invalid configuration file");

    let structure = parse_s3(&config.endpoints[0])
        .await
        .expect("Failed to parse structure");
}
