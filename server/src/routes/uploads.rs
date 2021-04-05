use rocket::{http::ContentType, Data};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::{
    fs::File,
    io::{self, Read},
};
use uuid::Uuid;

use crate::{
    auth::Auth,
    constants::{MAX_IMAGE_SIZE_BYTES, STATIC_FILES_BASE_PATH},
    models::{ContentLength, ErrorMessage},
};

#[derive(Serialize)]
pub struct FileMeta {
    url: String,
}

impl FileMeta {
    pub fn new(url: String) -> Self {
        FileMeta { url }
    }
}

#[post("/uploads", data = "<data>")]
pub fn upload_post(
    auth: Auth,
    content_type: &ContentType,
    content_length: Option<ContentLength>,
    data: Data,
) -> Result<Json<FileMeta>, ErrorMessage> {
    if let Err(e) = auth {
        return Err(e.into());
    }

    if let Some(length) = content_length {
        if !length.fits(MAX_IMAGE_SIZE_BYTES) {
            return Err(ErrorMessage::new(
                0,
                "File is too large".to_string(),
                "".to_string(),
            ));
        }
    } else {
        return Err(ErrorMessage::new(
            0,
            "Content-Length header must be supplied".to_string(),
            "".to_string(),
        ));
    }

    if content_type.is_png() || content_type.is_jpeg() {
        let file_id = Uuid::new_v4().to_simple().to_string();
        let ext = content_type.sub().as_str();
        let file_url = format!("{}/{}.{}", STATIC_FILES_BASE_PATH, file_id, ext);
        let manifest_dir = env!("CARGO_MANIFEST_DIR", "..");

        io::copy(
            &mut data.open().take(MAX_IMAGE_SIZE_BYTES), // truncate file in case it passed request guard
            &mut File::create(format!("{}{}", manifest_dir, file_url))?,
        )?;

        Ok(Json(FileMeta::new(file_url)))
    } else {
        Err(ErrorMessage::new(
            0,
            "Usupported file type".to_string(),
            "".to_string(),
        ))
    }
}
