// time units
const MINUTE: i32 = 60; // secs
const HOUR: i32 = 60 * MINUTE;
// const DAY: i32 = 24 * HOUR;

// auth related
pub const TOKEN_PREFIX: &'static str = "Bearer ";
pub const TOKEN_EXP: i64 = 15; // mins

// chrono related
pub const UTC_PLUS_THREE: i32 = 3 * HOUR;
pub const DATETIME_FORMAT_ARTICLE: &'static str = "%d %B %Y, %H:%M";
