use rand::Rng;
use serde::{Deserialize, Serialize};

use super::episode::Episode;

#[derive(Serialize, Deserialize)]
pub struct Season {
    name: String,
    number: i32,
    id: i32,
    episodes: Vec<Episode>,
    air_date: Option<String>,
    thumbnail: Option<String>,
}

impl Season {
    fn create(number: i32) -> Season {
        let mut rng = rand::thread_rng();
        Season {
            name: format!("Season {}", number),
            number: number,
            id: rng.gen_range(100000..1000000),
            episodes: vec![],
            air_date: None,
            thumbnail: None,
        }
    }
}
