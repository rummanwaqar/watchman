use std::{thread, time};
use vision_processor::*;

fn main() -> opencv::Result<()> {
    let config = configuration::get_configuration().expect("Unable to load configuration file");
    let mut camera = camera::Camera::new(config.camera)?;
    let mut motion_detector = motion_detector::MotionDetector::new(config.motion_detector);
    let mut recorder = None;

    loop {
        let frame = camera.read()?;
        let (_mat, motion_detected) = motion_detector.process(&frame)?;

        if recorder.is_none() {
            if motion_detected {
                println!("Motion detected, starting recording.");
                recorder = Some(recorder::Recorder::new(&config.recorder));
            }
        }

        if recorder.is_some() {
            let rec_ref = recorder.as_mut().unwrap();
            rec_ref.add_frame(&frame);
            if rec_ref.completed() {
                let filename = recorder::write_files(rec_ref);
                mailer::send_notification(&config.mailer, &filename).expect("Failed to send mail");
                recorder = None;
            }
        }

        thread::sleep(time::Duration::from_millis(66));
    }
    Ok(())
}
