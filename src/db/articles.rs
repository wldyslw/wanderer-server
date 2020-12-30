use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::article::*;
use crate::schema::{articles, users};

pub fn all(connection: &PgConnection) -> Vec<ArticleJson> {
    let articles = articles::table
        .inner_join(users::table)
        .select((articles::all_columns, users::username))
        .load::<(Article, String)>(connection)
        .ok();
    match articles {
        Some(a) => a
            .into_iter()
            .map(|(article, author_name)| article.to_json(author_name))
            .collect(),
        None => Vec::new(),
    }
}

pub fn find(connection: &PgConnection, slug: String) -> Option<ArticleJson> {
    let (article, author_name) = articles::table
        .inner_join(users::table)
        .select((articles::all_columns, users::username))
        .filter(articles::slug.eq(slug))
        .first::<(Article, String)>(connection)
        .ok()?;
    Some(article.to_json(author_name))
}
