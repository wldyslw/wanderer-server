// routing
pub const API_V1_BASE_PATH: &'static str = "/api/v1";
pub const STATIC_FILES_BASE_PATH: &'static str = "/static";

// upload related
pub const MAX_IMAGE_SIZE_BYTES: u64 = 10 * 1024 * 1024;

// time units
const MINUTE: i32 = 60; // secs
const HOUR: i32 = 60 * MINUTE;
const DAY: i32 = 24 * HOUR;

// auth related
pub const AUTH_COOKIE_NAME: &'static str = "session_token";
pub const ACCESS_TOKEN_EXP: i64 = (15 * DAY) as i64;

// chrono display related
pub const UTC_PLUS_THREE: i32 = 3 * HOUR;
pub const DATETIME_FORMAT_ARTICLE: &'static str = "%d %B %Y, %H:%M";
