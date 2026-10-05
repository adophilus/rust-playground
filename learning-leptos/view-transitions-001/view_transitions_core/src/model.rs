use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogTags(pub Vec<String>);

impl From<String> for BlogTags {
    fn from(value: String) -> Self {
        return Self(serde_json::from_str::<Vec<String>>(value.as_str()).unwrap());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub content: String,
    pub cover_image_url: Option<String>,
    pub source_url: String,
    pub tags: BlogTags,
}
