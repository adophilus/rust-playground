use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

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

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        return fmt.write_str(self.message.as_ref());
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return self.source.as_ref().map(|v| &**v);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedMeta {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T: Clone + Debug> {
    pub items: Vec<T>,
    pub meta: PaginatedMeta,
}

impl<T: Clone + std::fmt::Debug> Paginated<T> {
    pub fn new(items: Vec<T>, meta: PaginatedMeta) -> Self {
        return Self { items, meta };
    }
}
