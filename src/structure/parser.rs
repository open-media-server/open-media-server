use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Node {
    pub name: String,
    pub path: String,
    pub children: Option<Vec<Node>>,
}
