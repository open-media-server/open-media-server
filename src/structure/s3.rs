use std::vec;

use regex::Regex;
use s3::{creds::Credentials, error::S3Error, Bucket, Region};

use crate::config::config::Endpoint;

use super::parser::Node;

pub async fn parse_s3(endpoint: &Endpoint) -> Result<Node, S3Error> {
    let bucket = Bucket::new(
        &endpoint.bucket_name,
        Region::Custom {
            region: endpoint.region.clone(),
            endpoint: endpoint.url.clone(),
        },
        Credentials::default().expect("Failed to load s3 credentials"),
    )?;

    let res = bucket.list(endpoint.path.clone(), None).await?;

    let regex = Regex::new(".(swf|avi|flv|mpg|rm|mov|wav|asf|3gp|mkv|rmvb|mp4)").unwrap();

    let paths = res[0]
        .contents
        .iter()
        .map(|c| c.key.as_str())
        .filter(|c| regex.is_match(c))
        .collect::<Vec<&str>>();

    Ok(paths_to_structure(paths))
}

fn paths_to_structure(paths: Vec<&str>) -> Node {
    let mut root_node = Node {
        name: String::from(""),
        path: String::from(""),
        children: None,
    };

    for path in paths {
        let mut curr_node = &mut root_node;

        let parts = path.split("/");
        for part in parts {
            if curr_node.children.is_none() {
                curr_node.children = Some(vec![]);
            }

            let children = curr_node.children.as_mut().unwrap();

            let node = match children.iter().position(|n| n.name == part) {
                Some(i) => &mut children[i],
                None => {
                    let node = Node {
                        name: part.to_string(),
                        path: format!("{}/{}", curr_node.path, part),
                        children: None,
                    };
                    children.push(node);
                    children.last_mut().unwrap()
                }
            };

            curr_node = node;
        }
    }

    root_node
}
