use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::DATETIME_FORMAT_ARTICLE;

/// Represents article's DB model
#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub title_image: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorites_count: i32,
    pub author: i32,
}

/// Represents articles's JSON model sent over network with GET response
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleJson {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub title_image: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub favorites_count: i32,
    pub author_name: String,
}

impl Article {
    pub fn to_json(self, author_name: String) -> ArticleJson {
        ArticleJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            title_image: self.title_image,
            description: self.description,
            body: self.body,
            tag_list: self.tag_list,
            created_at: self
                .created_at
                .format_localized(DATETIME_FORMAT_ARTICLE, Locale::ru_RU)
                .to_string(),
            updated_at: self
                .updated_at
                .format_localized(DATETIME_FORMAT_ARTICLE, Locale::ru_RU)
                .to_string(),
            favorites_count: self.favorites_count,
            author_name,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleNew {
    pub slug: String,
    pub title: String,
    pub title_image: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}
