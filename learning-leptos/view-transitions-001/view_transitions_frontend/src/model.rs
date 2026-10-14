use leptos_router::{Params, ParamsError, ParamsMap};
use serde::Serialize;
use std::sync::Arc;
use view_transitions_core::model::{Blog, BlogTags};

#[derive(Debug)]
pub struct Error<'a> {
    pub message: &'a str,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
    }
}

impl<'a> std::error::Error for Error<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|v| &**v as &dyn std::error::Error)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(transparent)]
pub struct BlogQuery(pub Blog);

impl TryFrom<BlogQuery> for ParamsMap {
    type Error = String;

    fn try_from(value: BlogQuery) -> Result<Self, Self::Error> {
        let mut map = ParamsMap::new();

        map.insert("id".to_string(), value.0.id);
        map.insert("title".to_string(), value.0.title);
        map.insert("content".to_string(), value.0.content);
        if let Some(cover_image_url) = value.0.cover_image_url {
            map.insert("cover_image_url".to_string(), cover_image_url);
        }
        map.insert("source_url".to_string(), value.0.source_url);
        serde_json::to_string(&value.0.tags)
            .map(|v| {
                map.insert("tags".to_string(), v);
            })
            .map_err(|_| "Failed to insert tags into ParamsMap")?;

        Ok(map)
    }
}

impl Params for BlogQuery {
    fn from_map(map: &leptos_router::ParamsMap) -> Result<Self, leptos_router::ParamsError> {
        let id = map.get("id").ok_or(ParamsError::Params(Arc::new(Error {
            message: "id not in ParamsMap",
            source: None,
        })))?;
        let title = map.get("title").ok_or(ParamsError::Params(Arc::new(Error {
            message: "title not in ParamsMap",
            source: None,
        })))?;
        let content = map
            .get("content")
            .ok_or(ParamsError::Params(Arc::new(Error {
                message: "content not in ParamsMap",
                source: None,
            })))?;
        let cover_image_url = map.get("cover_image_url").map(|v| v.clone());
        let source_url = map
            .get("source_url")
            .ok_or(ParamsError::Params(Arc::new(Error {
                message: "source_url not in ParamsMap",
                source: None,
            })))?;
        let tags = map
            .get("tags")
            .map(|v| {
                serde_json::from_str::<BlogTags>(v).map_err(|e| {
                    ParamsError::Params(Arc::new(Error {
                        message: "invalid tags in ParamsMap",
                        source: Some(Box::new(e)),
                    }))
                })
            })
            .ok_or(ParamsError::Params(Arc::new(Error {
                message: "tags not in ParamsMap",
                source: None,
            })))
            .flatten()?;

        Ok(Self(Blog {
            id: id.clone(),
            title: title.clone(),
            content: content.clone(),
            cover_image_url,
            source_url: source_url.clone(),
            tags: tags.clone(),
        }))
    }
}
