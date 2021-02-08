use chrono::prelude::*;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::schema::articles;

use super::util::locale_string;

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
    pub is_draft: bool,
}

/// Represents articles's JSON model sent over network with GET response
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleGet {
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
    pub is_draft: bool,
}

impl Article {
    pub fn to_json(&self, author_name: String) -> ArticleGet {
        ArticleGet {
            id: self.id,
            slug: self.slug.clone(),
            title: self.title.clone(),
            title_image: self.title_image.clone(),
            description: self.description.clone(),
            body: self.body.clone(),
            tag_list: self.tag_list.clone(),
            created_at: locale_string(self.created_at),
            updated_at: locale_string(self.updated_at),
            favorites_count: self.favorites_count,
            author_name,
            is_draft: self.is_draft,
        }
    }
}

impl<'a> Responder<'a> for ArticleGet {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        Json(self).respond_to(req)
    }
}

/// Represents article's JSON model sent network in order to create new article
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleNew {
    pub slug: String,
    pub title: String,
    pub title_image: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub is_draft: bool,
}

/// Represents article's DB model suitable for insertion in DB and creating new article
#[derive(Insertable)]
#[table_name = "articles"]
pub struct ArticleInsertable<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub title_image: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub tag_list: &'a Vec<String>,
    pub author: i32,
    pub is_draft: bool,
}

impl ArticleNew {
    pub fn to_insertable(&self, author: i32) -> ArticleInsertable {
        ArticleInsertable {
            slug: &self.slug,
            title: &self.title,
            title_image: &self.title_image,
            description: &self.description,
            body: &self.body,
            tag_list: &self.tag_list,
            author,
            is_draft: self.is_draft,
        }
    }
}

/// Represents both article's JSON and DB models used in the process of updating article
#[derive(Deserialize, AsChangeset)]
#[table_name = "articles"]
#[serde(rename_all = "camelCase")]
pub struct ArticleUpdate {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub title_image: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
    pub is_draft: Option<bool>,
}
