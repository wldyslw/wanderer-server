use rocket::Request;
use rocket_contrib::json::JsonValue;

// TODO: supply more info in catchers

#[catch(404)]
pub fn not_found(_req: &Request) -> JsonValue {
    json!({ "status": 404 })
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> JsonValue {
    json!({ "status": 403 })
}
