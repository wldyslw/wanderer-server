use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::article::*;
use crate::schema::articles;

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
