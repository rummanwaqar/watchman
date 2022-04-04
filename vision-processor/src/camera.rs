use crate::helpers::resize;
use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use opencv::videoio::CAP_ANY;

pub struct Camera {
    cap: VideoCapture,
    frame_count: i64,
    settings: Settings,
}

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    camera_id: i32,
    size: [i32; 2],
}

impl Camera {
    pub fn new(settings: Settings) -> Result<Self, opencv::Error> {
        Ok(Camera {
            cap: VideoCapture::new(settings.camera_id, CAP_ANY)?,
            frame_count: 0,
            settings,
        })
    }

    pub fn read(&mut self) -> Result<Mat, opencv::Error> {
        let mut frame = Mat::default();
        self.cap.read(&mut frame)?;
        self.frame_count += 1;
        resize(&frame, self.settings.size)
    }
}
