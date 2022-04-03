use chrono::Utc;
use opencv::core::absdiff;
use opencv::highgui::wait_key;
use opencv::imgcodecs::imread;
use opencv::ximgproc::threshold;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, types, videoio, Result};
use std::fs::DirEntry;
use std::{env, fs};

use vision_processor::helpers::{add, apply_mask, bitwise_not, bitwise_or, weighted_sum};
use vision_processor::recorder::{write_files, Recorder};
use vision_processor::{background_subtraction, recorder};
use vision_processor::{mailer, temporal_difference};

fn main() -> Result<()> {
    let camera_id = 4;
    let mut cam = videoio::VideoCapture::new(camera_id, videoio::CAP_ANY)?;

    let mut frame = Mat::default();

    let mut count = 0;

    let mut r = Recorder::new(recorder::Settings {
        storage_path: "/home/rumman/Videos/".to_string(),
        video_length: 20,
    });

    loop {
        cam.read(&mut frame)?;
        r.add_frame(&frame);
        if r.completed() {
            let filename = write_files(&r);
            mailer::send_notification(
                mailer::Settings {
                    from: "rumman.waqar05@gmail.com".to_string(),
                    to: "waqar@ualberta.ca".to_string(),
                    username: "rumman.waqar05@gmail.com".to_string(),
                    password: "hxb-xfg!egb!AJB8cpm".to_string(),
                    smtp: "smtp.gmail.com".to_string(),
                },
                &filename,
            );
            break;
        }
        // highgui::imshow("bg_sub", &frame)?;
        highgui::wait_key(1)?;
    }
    Ok(())
}

// start saving
// create thumbnail
// repeat for 20 secs
// save video and thumbnail
// send email notification

fn process() -> Result<()> {
    let path = env::current_dir()
        .unwrap()
        .join("data")
        .join("PETS2006")
        .join("input");

    let files = get_file_list(path.to_str().unwrap());

    highgui::named_window("input", highgui::WINDOW_FULLSCREEN)?;
    highgui::named_window("temporal", highgui::WINDOW_FULLSCREEN)?;
    highgui::named_window("bg_sub", highgui::WINDOW_FULLSCREEN)?;
    highgui::named_window("combined_mask", highgui::WINDOW_FULLSCREEN)?;

    let mut temporal_difference =
        temporal_difference::TemporalDifference::new(temporal_difference::Settings {
            blur_size: 5,
            thresh: 60.0,
        });
    let mut bg_sub =
        background_subtraction::BackgroundSubtraction::new(background_subtraction::Settings {
            thresh: 55.0,
            blur_size: 5,
            alpha: 0.01,
        });

    let camera_id = 4;
    let mut cam = videoio::VideoCapture::new(camera_id, videoio::CAP_ANY)?;

    let mut frame = Mat::default();

    let mut count = 0;
    // for image_file in files {
    loop {
        // let image = imread(&image_file, imgcodecs::IMREAD_GRAYSCALE)?;
        cam.read(&mut frame)?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_RGB2GRAY, 0);

        let mut image = Mat::default();
        imgproc::resize(
            &gray,
            &mut image,
            core::Size::default(),
            0.3,
            0.3,
            imgproc::INTER_AREA,
        );

        let bg_sub_output = bg_sub.update(&image)?;
        let temporal_output = temporal_difference.process(&image)?;

        if temporal_output.is_some() && bg_sub_output.is_some() {
            let bg_sub = bg_sub_output.as_ref().unwrap();
            let temporal_sub = temporal_output.as_ref().unwrap();

            highgui::imshow("bg_sub", bg_sub)?;
            highgui::imshow("temporal", &temporal_sub)?;

            let mask = bitwise_or(bg_sub, bg_sub)?;
            highgui::imshow("combined_mask", &mask)?;

            // bounding boxes
            let mut vec = types::VectorOfMat::new();
            imgproc::find_contours(
                &mask,
                &mut vec,
                opencv::imgproc::RETR_EXTERNAL,
                opencv::imgproc::CHAIN_APPROX_NONE,
                opencv::core::Point::new(0, 0),
            )?;

            let mut output = image.clone();
            for contour in vec {
                let area = imgproc::contour_area(&contour, false)?;
                let rect = opencv::imgproc::bounding_rect(&contour)?;
                opencv::imgproc::rectangle(
                    &mut output,
                    rect,
                    core::Scalar::new(255., 0., 0., 255.),
                    1,
                    imgproc::LINE_AA,
                    0,
                );
            }
            highgui::imshow("input", &output)?;
        }
        count += 1;
        println!("{}", count);
        highgui::wait_key(1)?;
    }

    Ok(())
}

fn get_file_list(path: &str) -> Vec<String> {
    let mut files = vec![];
    for entry in fs::read_dir(path).unwrap() {
        files.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    files.sort();
    files
}
