use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Episode {
    name: String,
    number: i32,
    id: i32,
    path: String,
    thumbnail: Option<String>,
}

impl Episode {
    fn create(name: &str, number: i32, path: &str) -> Episode {
        let mut rng = rand::thread_rng();
        Episode {
            number,
            id: rng.gen_range(100000..1000000),
            name: name.to_string(),
            path: path.to_string(),
            thumbnail: None,
        }
    }
}
