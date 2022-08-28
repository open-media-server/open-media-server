use rand::Rng;
use serde::{Deserialize, Serialize};

use super::season::Season;

#[derive(Serialize, Deserialize)]
pub struct Show {
    name: String,
    seasons: Vec<Season>,
    id: i32,
    description: Option<String>,
    original_name: Option<String>,
    air_date: Option<String>,
    rating: Option<i32>,
    thumbnail: Option<String>,
}

impl Show {
    fn create(name: &str) -> Show {
        let mut rng = rand::thread_rng();
        Show {
            name: name.to_string(),
            seasons: vec![],
            id: rng.gen_range(100000..1000000),
            description: None,
            original_name: None,
            air_date: None,
            rating: None,
            thumbnail: None,
        }
    }
}
