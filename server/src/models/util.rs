use chrono::prelude::*;

use crate::constants::{DATETIME_FORMAT_ARTICLE, UTC_PLUS_THREE};

pub fn locale_string(date_time: DateTime<Utc>) -> String {
    date_time
        .with_timezone(&FixedOffset::east(UTC_PLUS_THREE))
        .format_localized(DATETIME_FORMAT_ARTICLE, Locale::ru_RU)
        .to_string()
}
