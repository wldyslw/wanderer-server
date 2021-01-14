use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::{article::*, user::User, ErrorMessage};
use crate::schema::{articles, users};

pub fn all(connection: &PgConnection) -> Vec<ArticleGet> {
    let articles = articles::table
        .inner_join(users::table)
        .select((articles::all_columns, users::username))
        .load::<(Article, String)>(connection);
    match articles {
        Ok(a) => a
            .into_iter()
            .map(|(article, author_name)| article.to_json(author_name))
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn find(connection: &PgConnection, slug: String) -> Result<ArticleGet, ErrorMessage> {
    articles::table
        .inner_join(users::table)
        .select((articles::all_columns, users::username))
        .filter(articles::slug.eq(slug))
        .first::<(Article, String)>(connection)
        .map(|(article, author_name)| article.to_json(author_name))
        .map_err(|e| e.into())
}

pub fn new(
    connection: &PgConnection,
    article: ArticleNew,
    author_id: i32,
) -> Result<ArticleGet, ErrorMessage> {
    diesel::insert_into(articles::table)
        .values(&article.to_insertable(author_id))
        .get_result::<Article>(connection)
        .map(|a| a.populate(connection))
        .map_err(|e| e.into())
}

pub fn update(
    connection: &PgConnection,
    slug: String,
    article: ArticleUpdate,
) -> Result<ArticleGet, ErrorMessage> {
    let source = articles::table.filter(articles::slug.eq(slug));
    diesel::update(source)
        .set(&article)
        .get_result::<Article>(connection)
        .map(|a| a.populate(connection))
        .map_err(|e| e.into())
}

impl Article {
    fn populate(&self, connection: &PgConnection) -> ArticleGet {
        let author: User = users::table
            .find(self.author)
            .get_result::<User>(connection)
            .expect("Error loading author");

        self.to_json(author.username)
    }
}
