use super::model::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CLone)]
pub struct TagResponse {
    pub tags: Vec<String>,
}

impl TagsResponse {
    pub fn from_tags(tags: Vec<Tag>) -> Self {

        // This line creates an iterator over tags, transforms each Tag 
        // into its name field using the map method, and then collects 
        // the results into a new collection list. The type of list is 
        // inferred by Rust, and it will be a collection of the same type 
        // as the name field of Tag.
        let list = tags.iter().map(|tag| tag.name).collect();
        Self { tags: list }
    }
}