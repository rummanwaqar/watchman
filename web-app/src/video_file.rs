use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
pub struct VideoFile {
    pub filename: String,
    pub date_time: String,
    pub path: String,
}

impl VideoFile {
    pub fn new(filename: &str, base_path: &str) -> Option<Self> {
        let path = Path::new(base_path).join(filename);
        if !path.exists() || path.extension().unwrap() != "mp4" {
            return None;
        }
        let date_time: DateTime<Utc> = fs::metadata(&path).unwrap().created().unwrap().into();
        Some(VideoFile {
            filename: filename.to_string(),
            date_time: date_time.format("%v %r").to_string(),
            path: path.as_path().to_str().unwrap().to_string(),
        })
    }
}
