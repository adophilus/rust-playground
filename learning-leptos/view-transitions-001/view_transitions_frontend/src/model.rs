use leptos_router::{Params, ParamsError, ParamsMap};
use serde::Serialize;
use std::sync::Arc;
use view_transitions_core::model::Blog;

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

        map.insert("id".to_string(), value.0.id)
            .ok_or("Failed to insert id into ParamsMap")?;
        map.insert("title".to_string(), value.0.title)
            .ok_or("Failed to insert title into ParamsMap")?;
        map.insert("content".to_string(), value.0.content)
            .ok_or("Failed to insert content into ParamsMap")?;
        if let Some(cover_image_url) = value.0.cover_image_url {
            map.insert("cover_image_url".to_string(), cover_image_url)
                .ok_or("Failed to insert cover_image_url into ParamsMap")?;
        }
        map.insert("source_url".to_string(), value.0.source_url)
            .ok_or("Failed to insert source_url into ParamsMap")?;
        serde_json::to_string(&value.0.tags)
            .map(|v| {
                map.insert("tags".to_string(), v)
                    .ok_or("Failed to insert tags into ParamsMap")
            })
            .map_err(|_| "Failed to insert tags into ParamsMap")
            .flatten()?;

        Ok(map)
    }
}

impl Params for BlogQuery {
    fn from_map(map: &leptos_router::ParamsMap) -> Result<Self, leptos_router::ParamsError> {
        serde_json::json!(map)
            .as_str()
            .map(|v| {
                serde_json::from_str::<Blog>(v)
                    .map(|v| Self(v))
                    .map_err(|e| {
                        ParamsError::Params(Arc::new(Error {
                            message: "Invalid query",
                            source: Some(Box::new(e)),
                        }))
                    })
            })
            .ok_or(ParamsError::Params(Arc::new(Error {
                message: "Missing query",
                source: None,
            })))
            .flatten()
    }
}
