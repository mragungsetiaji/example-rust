use super::model::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

impl std::convert::From<Vec<Tag>> for TagsResponse {
    fn from(tags: Vec<Tag>) -> Self {

        // This line creates an iterator over tags, transforms each Tag 
        // into its name field using the map method, and then collects 
        // the results into a new collection list. The type of list is 
        // inferred by Rust, and it will be a collection of the same type 
        // as the name field of Tag.
        let list = tags.iter().map(|tag| tag.name.clone()).collect();
        Self { tags: list }
    }
}