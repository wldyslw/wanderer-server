use rocket_contrib::json::JsonValue;

use crate::db;
use crate::db::DBConnection;

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
