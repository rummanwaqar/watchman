use opencv::core::absdiff;
use opencv::highgui::wait_key;
use opencv::imgcodecs::imread;
use opencv::ximgproc::threshold;
use opencv::{highgui, imgcodecs, imgproc, prelude::*, videoio, Result};
use std::env;

use vision_processor::background_subtraction;
use vision_processor::helpers::{add, apply_mask, bitwise_not, weighted_sum};
use vision_processor::temporal_difference;

fn main() -> Result<()> {
    let path = env::current_dir()
        .unwrap()
        .join("data")
        .join("office")
        .join("input");

    let image1 = imread(
        path.join("in000001.jpg").to_str().unwrap(),
        imgcodecs::IMREAD_GRAYSCALE,
    )?;
    let image2 = imread(
        path.join("in000002.jpg").to_str().unwrap(),
        imgcodecs::IMREAD_GRAYSCALE,
    )?;
    let image3 = imread(
        path.join("in000750.jpg").to_str().unwrap(),
        imgcodecs::IMREAD_GRAYSCALE,
    )?;
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;

    // let mut temporal_difference = temporal_difference::TemporalDifference::new();
    // let settings = temporal_difference::Settings {
    //     blur_size: 5,
    //     thresh: 60.0,
    // };
    // let output = temporal_difference.process(&image1, settings)?;
    // if output.is_none() {
    //     println!("first image good");
    // }
    // let output = temporal_difference.process(&image2, settings)?;
    // if output.is_none() {
    //     println!("second image good");
    // }
    // let output = temporal_difference.process(&image3, settings)?;
    // if output.is_some() {
    //     println!("got something for image 3");
    //
    //     highgui::imshow("window", &output.as_ref().unwrap())?;
    //     wait_key(0)?;
    // }

    let kernel = opencv::core::Mat::ones(3, 3, 1)?.to_mat()?;
    println!("{:?}", kernel);

    let mut bg_sub = background_subtraction::BackgroundSubtraction::new();
    let settings = background_subtraction::Settings {
        thresh: 50.0,
        blur_size: 3,
    };
    bg_sub.update_background(&image1, &Mat::default(), settings)?;
    let output = bg_sub.calculate_difference(&image3, settings)?;
    if output.is_some() {
        let mask = output.as_ref().unwrap();



        let new_pixels = apply_mask(&image3, &bitwise_not(mask)?)?;
        let old_pixels = apply_mask(&image1, mask)?;

        let new_update = add(&new_pixels, &old_pixels)?;

        let new_bg = weighted_sum(&new_update, &image1, 0.8)?;

        highgui::imshow("window", &new_update)?;
        wait_key(0)?;
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
