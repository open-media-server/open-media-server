use serde::{Deserialize, Serialize};

use super::show::Show;

#[derive(Serialize, Deserialize)]
pub struct MediaConfig {
    base_url: String,
    media: Vec<Show>,
}
