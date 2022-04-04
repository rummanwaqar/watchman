use chrono::{NaiveTime, Utc};
use opencv::core::Mat;
use opencv::prelude::*;
use std::path::Path;

pub struct Recorder {
    frames: Vec<Mat>,
    start_time: NaiveTime,
    completed: bool,
    settings: Settings,
}

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub storage_path: String,
    pub video_length: i64,
}

impl Recorder {
    pub fn new(settings: &Settings) -> Self {
        Recorder {
            frames: vec![],
            start_time: Utc::now().time(),
            completed: false,
            settings: Settings {
                storage_path: settings.storage_path.clone(),
                video_length: settings.video_length,
            },
        }
    }

    pub fn add_frame(&mut self, frame: &Mat) {
        let diff = Utc::now().time() - self.start_time;
        if diff.num_seconds() >= self.settings.video_length {
            self.completed = true;
            return;
        }
        self.frames.push(frame.clone());
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

pub fn write_files(recorder: &Recorder) -> String {
    if !recorder.completed {
        return "".to_string();
    }

    let filename = Utc::now().date().format("%F-").to_string()
        + &*recorder.start_time.format("%H-%M-%S").to_string();
    let filepath = Path::new(&recorder.settings.storage_path)
        .join(filename.clone())
        .to_str()
        .unwrap()
        .to_string();

    // write video
    let codec =
        opencv::videoio::VideoWriter::fourcc('a' as i8, 'v' as i8, 'c' as i8, '1' as i8).unwrap();
    let framerate = recorder.frames.len() as f64 / recorder.settings.video_length as f64;
    let frame_size = opencv::core::Size::new(recorder.frames[0].cols(), recorder.frames[0].rows());
    let mut writer = opencv::videoio::VideoWriter::new(
        &*(filepath.clone() + ".mp4"),
        codec,
        framerate,
        frame_size,
        true,
    )
    .expect("File writer failed to work correctly");
    for frame in &recorder.frames {
        writer
            .write(frame)
            .expect("Failed to write to video writer");
    }
    writer.release().expect("Failed to release video writer");

    // write thumbnail
    opencv::imgcodecs::imwrite(
        &*(filepath.clone() + ".png"),
        &recorder.frames[0],
        &opencv::core::Vector::new(),
    )
    .expect("Failed to create thumbnail");

    println!("{} written with {} frames", filepath, recorder.frames.len());
    filename + ".mp4"
}
