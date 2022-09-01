use std::vec;

use serde::{Deserialize, Serialize};
use torrent_name_parser::Metadata;

use crate::{config::config::Endpoint, structure::parser::Node};

use super::{
    episode::{parse_episode_number, Episode},
    season::{parse_season_number, Season},
    show::Show,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub base_url: String,
    pub media: Vec<Show>,
}

impl Library {
    pub fn from_structure(endpoint: &Endpoint, root_node: &Node) -> Self {
        let mut library = Library {
            base_url: format!(
                "{}/{}/{}",
                endpoint.url, endpoint.bucket_name, endpoint.path,
            ),
            media: vec![],
        };

        for show_folder in match root_node.children.as_ref() {
            Some(children) => children,
            None => return library,
        } {
            let mut show = parse_show(&mut library, show_folder);

            for season_folder in match show_folder.children.as_ref() {
                Some(children) => children,
                None => continue,
            } {
                let mut season = parse_season(&mut show, season_folder);

                if season_folder.children.is_none() {
                    parse_episode(season, season_folder);
                    continue;
                }

                for episode_file in match season_folder.children.as_ref() {
                    Some(children) => children,
                    None => continue,
                } {
                    parse_episode(&mut season, episode_file);
                }
            }
        }

        library
    }
}

fn parse_show<'a>(library: &'a mut Library, show_folder: &'a Node) -> &'a mut Show {
    let mut name = show_folder.name.clone();
    if let Ok(metadata) = Metadata::from(&name) {
        name = metadata.title().to_string();
    }

    let index = match library.media.iter().position(|s| s.name.as_str() == name) {
        Some(i) => i,
        None => {
            let show = Show::create(&name);
            library.media.push(show);
            library.media.len() - 1
        }
    };

    &mut library.media[index]
}

fn parse_season<'a>(show: &'a mut Show, season_folder: &'a Node) -> &'a mut Season {
    let number = parse_season_number(&season_folder.name).unwrap_or(1);

    let index = match show.seasons.iter().position(|s| s.number == number) {
        Some(i) => i,
        None => {
            let season = Season::create(number);
            show.seasons.push(season);
            show.seasons.len() - 1
        }
    };

    &mut show.seasons[index]
}

fn parse_episode(season: &mut Season, episode_file: &Node) {
    let number = parse_episode_number(&episode_file.name).unwrap_or(0);

    season
        .episodes
        .push(Episode::create("", number, &episode_file.path));
}
