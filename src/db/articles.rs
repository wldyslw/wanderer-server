use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;

use crate::config::DATETIME_FORMAT_ARTICLE;
use crate::schema::articles;

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
}

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
}

impl Into<ArticleJson> for Article {
    fn into(self) -> ArticleJson {
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
                .format_localized(DATETIME_FORMAT_ARTICLE, Locale::be_BY)
                .to_string(),
            updated_at: self
                .updated_at
                .format_localized(DATETIME_FORMAT_ARTICLE, Locale::be_BY)
                .to_string(),
            favorites_count: self.favorites_count,
        }
    }
}

pub fn all(connection: &PgConnection) -> Vec<ArticleJson> {
    let articles = articles::table
        .select(articles::all_columns)
        .load::<Article>(connection)
        .ok();
    match articles {
        Some(a) => a.into_iter().map(|article| article.into()).collect(),
        None => Vec::new(),
    }
}

pub fn find(connection: &PgConnection, slug: String) -> Option<ArticleJson> {
    let article = articles::table
        .select(articles::all_columns)
        .filter(articles::slug.eq(slug))
        .first::<Article>(connection)
        .ok()?;
    Some(article.into())
}
