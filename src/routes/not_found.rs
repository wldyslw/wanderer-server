use rocket::Request;
use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found(_req: &Request) -> JsonValue {
    json!({ "status": 404 })
}
