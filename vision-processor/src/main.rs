use opencv::core::absdiff;
use opencv::highgui::wait_key;
use opencv::imgcodecs::imread;
use opencv::ximgproc::threshold;
use opencv::{highgui, imgcodecs, imgproc, prelude::*, videoio, Result};
use std::fs::DirEntry;
use std::{env, fs};

use vision_processor::background_subtraction;
use vision_processor::helpers::{add, apply_mask, bitwise_not, bitwise_or, weighted_sum};
use vision_processor::temporal_difference;

fn main() -> Result<()> {
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
            thresh: 50.0,
            blur_size: 3,
            alpha: 0.01,
        });

    let mut count = 0;
    for image_file in files {
        let image = imread(&image_file, imgcodecs::IMREAD_GRAYSCALE)?;
        let bg_sub_output = bg_sub.update(&image)?;
        let temporal_output = temporal_difference.process(&image)?;

        highgui::imshow("input", &image)?;
        if temporal_output.is_some() && bg_sub_output.is_some() {
            let bg_sub = bg_sub_output.as_ref().unwrap();
            let temporal_sub = temporal_output.as_ref().unwrap();

            highgui::imshow("bg_sub", bg_sub)?;
            highgui::imshow("temporal", &temporal_sub)?;

            let mask = bitwise_or(bg_sub, bg_sub)?;
            highgui::imshow("combined_mask", &mask)?;
        }
        count += 1;
        println!("{}", count);
        highgui::wait_key(0)?;
    }

    // let camera_id = 4;
    // let mut cam = videoio::VideoCapture::new(camera_id, videoio::CAP_ANY)?;

    // let mut frame = Mat::default();
    //
    // loop {
    //     cam.read(&mut frame)?;
    //     highgui::imshow("window", &frame)?;
    //
    //     let key = highgui::wait_key(1)?;
    //     if key == 113 {
    //         break;
    //     }
    // }

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
