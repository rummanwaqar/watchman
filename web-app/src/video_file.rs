use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

#[derive(serde::Serialize, Debug)]
pub struct VideoFile {
    pub filename: String,
    pub date_time: String,
    pub path: String,
    pub image_name: String,
}

impl VideoFile {
    pub fn new(filename: &str, base_path: &str) -> Option<Self> {
        let path = Path::new(base_path).join(filename);
        if !path.exists() || path.extension().unwrap() != "mp4" {
            return None;
        }
        let mut image_path = path.clone();
        image_path.set_extension("png");
        if !image_path.exists() {
            return None;
        }
        let date_time: DateTime<Utc> = fs::metadata(&path).unwrap().created().unwrap().into();
        Some(VideoFile {
            filename: filename.to_string(),
            date_time: date_time.format("%v %r").to_string(),
            path: path.as_path().to_str().unwrap().to_string(),
            image_name: image_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        })
    }
}

pub fn get_videos_from_path(path: &str) -> Vec<VideoFile> {
    let mut output = Vec::new();
    if let Ok(files) = fs::read_dir(path) {
        for entry in files {
            if let Ok(entry) = entry {
                if let Some(video_file) =
                    VideoFile::new(entry.path().file_name().unwrap().to_str().unwrap(), path)
                {
                    output.push(video_file);
                }
            }
        }
    }
    output
}
