use rand::Rng;
use serde::{Deserialize, Serialize};

use super::season::Season;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub name: String,
    pub seasons: Vec<Season>,
    pub id: i32,
    pub description: Option<String>,
    pub original_name: Option<String>,
    pub air_date: Option<String>,
    pub rating: Option<i32>,
    pub thumbnail: Option<String>,
}

impl Show {
    pub fn create(name: &str) -> Show {
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
