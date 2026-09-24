use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub content: String,
    pub cover_image_url: Option<String>,
    pub banner_image_url: Option<String>,
    pub source_link_url: String,
    pub tags: Vec<String>,
}
