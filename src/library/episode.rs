use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub name: String,
    pub number: i32,
    pub id: i32,
    pub path: String,
    pub thumbnail: Option<String>,
}

impl Episode {
    pub fn create(name: &str, number: i32, path: &str) -> Episode {
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

pub fn parse_episode_number(name: &str) -> Option<i32> {
    let regex = Regex::new(
        r"(?i)(?:e|episode)[^.\d]?(?P<short>\d{1,3})|\d+x(?P<cross>\d+)|s\d+ - (?P<dash>\d+)",
    )
    .unwrap();

    let captures = regex.captures(name)?;

    captures
        .name("short")
        .or_else(|| captures.name("cross"))
        .or_else(|| captures.name("dash"))
        .map(|m| m.as_str().parse().ok())?
}
