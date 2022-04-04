use opencv::{highgui, Result};
use std::{env, fs, thread, time};

use vision_processor::recorder::*;
use vision_processor::*;

fn main() -> Result<()> {
    let config = configuration::get_configuration().expect("Unable to load configuration file");
    let mut camera = camera::Camera::new(config.camera)?;
    let mut motion_detector = motion_detector::MotionDetector::new(config.motion_detector);

    // let mut r = Recorder::new(config.recorder);
    highgui::named_window("combined_mask", highgui::WINDOW_FULLSCREEN)?;

    loop {
        let frame = camera.read()?;
        let (mat, motion_detected) = motion_detector.process(&frame)?;
        highgui::imshow("combined_mask", &mat);
        println!("{}", motion_detected);

        // r.add_frame(&frame);
        // if r.completed() {
        //     let filename = write_files(&r);
        //     mailer::send_notification(config.mailer, &filename);
        //     break;
        // }
        // thread::sleep(time::Duration::from_millis(66));
        opencv::highgui::wait_key(1)?;
    }
    Ok(())
}
