use crate::{
    define_paginated,
    model::{PaginatedItems, PaginatedMeta},
};
use serde::{Deserialize, Serialize};
use sqlx::SqliteExecutor;
use view_transitions_core::model::{Blog, Error, Paginated};

define_paginated!(Blog);

pub async fn list_blogs<'e, E: SqliteExecutor<'e>>(
    e: E,
    page: i32,
    per_page: i32,
) -> Result<Paginated<Blog>, Error> {
    return sqlx::query_as!(
        PaginatedBlog,
        r#"
        WITH _filtered_items AS (
            SELECT * FROM blogs
        ),
        _windowed_items AS (
            SELECT * FROM _filtered_items LIMIT $2 OFFSET $2 * ($1 - 1)
        ),
        _windowed_items_json AS (
            SELECT
                JSON_GROUP_ARRAY(
                    JSON_OBJECT(
                        'id', id,
                        'title', title,
                        'content', content,
                        'cover_image_url', cover_image_url,
                        'source_url', source_url,
                        'tags', JSON(tags)
                    )
                ) AS item
            FROM
                _windowed_items
        ),
        _total_count AS (
            SELECT COUNT(id) AS total FROM _filtered_items
        )
        SELECT
            JSON(COALESCE((SELECT item FROM _windowed_items_json), '[]')) AS "items!: sqlx::types::Json<PaginatedItems<Blog>>",
            JSON_OBJECT(
                'total', (SELECT total FROM _total_count),
                'page', $1,
                'per_page', $2
            ) AS "meta!: sqlx::types::Json<PaginatedMeta>"
        FROM
            _windowed_items
        "#,
        page,
        per_page
    )
    .fetch_one(e)
    .await
        .map(|v| -> Paginated<Blog> {v.into()})
        .map_err(|e| Error{message: String::from("Failed to list blogs"), source: Some(Box::new(e))});
}

pub async fn get_blog_by_id<'e, E: SqliteExecutor<'e>>(e: E, id: &str) -> Result<Blog, Error> {
    return sqlx::query_as!(Blog, "SELECT * FROM blogs WHERE Id = $1", id)
        .fetch_one(e)
        .await
        .map_err(|e| Error {
            message: String::from("Failed to get blog by id"),
            source: Some(Box::new(e)),
        });
}
