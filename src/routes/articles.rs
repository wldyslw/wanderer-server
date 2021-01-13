use rocket_contrib::json::{Json, JsonValue};

use crate::{
    auth::AuthClaims,
    db::{self, DBConnection},
    models::article::*,
};

#[get("/articles")]
pub fn articles_get(connection: DBConnection) -> JsonValue {
    let articles = db::articles::all(&connection);
    json!({ "articles": articles })
}

#[get("/articles/<slug>")]
pub fn article_get(connection: DBConnection, slug: String) -> JsonValue {
    let article = db::articles::find(&connection, slug);
    match article {
        Some(a) => json!({ "article": a }),
        None => json!({ "article": null }),
    }
}

#[post("/articles", format = "json", data = "<article>")]
pub fn article_create(
    connection: DBConnection,
    auth_claims: AuthClaims,
    article: Json<ArticleNew>,
) -> JsonValue {
    let result = db::articles::new(&connection, article.into_inner(), auth_claims.id);
    let status = if result.is_ok() { "ok" } else { "err" }; // TODO: implement proper error messages
    json!({ "status": status })
}

#[put("/articles/<slug>", format = "json", data = "<article>")]
pub fn article_update(
    connection: DBConnection,
    _auth_claims: AuthClaims,
    slug: String,
    article: Json<ArticleUpdate>,
) -> JsonValue {
    let result = db::articles::update(&connection, slug, article.into_inner());
    let status = if result.is_ok() { "ok" } else { "err" }; // TODO: implement proper error messages
    json!({ "status": status })
}
