use rocket_contrib::json::Json;

use crate::{
    auth::AuthClaims,
    db::{self, DBConnection},
    models::{article::*, ErrorMessage},
};

#[get("/articles")]
pub fn articles_get(connection: DBConnection) -> Json<Vec<ArticleGet>> {
    Json(db::articles::all(&connection))
}

#[get("/articles/<slug>")]
pub fn article_get(connection: DBConnection, slug: String) -> Result<ArticleGet, ErrorMessage> {
    db::articles::find(&connection, slug)
}

#[post("/articles", format = "json", data = "<article>")]
pub fn article_create(
    connection: DBConnection,
    auth_claims: AuthClaims,
    article: Json<ArticleNew>,
) -> Result<ArticleGet, ErrorMessage> {
    db::articles::new(&connection, article.into_inner(), auth_claims.id)
}

#[put("/articles/<slug>", format = "json", data = "<article>")]
pub fn article_update(
    connection: DBConnection,
    _auth_claims: AuthClaims,
    slug: String,
    article: Json<ArticleUpdate>,
) -> Result<ArticleGet, ErrorMessage> {
    db::articles::update(&connection, slug, article.into_inner())
}

#[put("/articles/<slug>/archive")]
pub fn article_archive(
    connection: DBConnection,
    _auth_claims: AuthClaims,
    slug: String,
) -> Result<ArticleGet, ErrorMessage> {
    // TODO: raise an error if article is already in archive
    db::articles::archive(&connection, slug)
}
