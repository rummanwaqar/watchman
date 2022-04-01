use opencv::core::absdiff;
use opencv::highgui::wait_key;
use opencv::imgcodecs::imread;
use opencv::ximgproc::threshold;
use opencv::{highgui, imgcodecs, imgproc, prelude::*, videoio, Result};
use std::env;

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

    let mut temporal_difference = vision_processor::temporal_difference::TemporalDifference::new();
    let settings = vision_processor::temporal_difference::Settings {
        blur_size: 5,
        thresh: 60.0
    };
    let output = temporal_difference.process(&image1, settings)?;
    if output.is_none() {
        println!("first image good");
    }
    let output = temporal_difference.process(&image2, settings)?;
    if output.is_none() {
        println!("second image good");
    }
    let output = temporal_difference.process(&image3, settings)?;
    if output.is_some() {
        println!("got something for image 3");

        highgui::imshow("window", &output.as_ref().unwrap())?;
        wait_key(0)?;
    }

    // let image3 = imread(path.join("in000003.jpg").to_str().unwrap(),
    //                     imgcodecs::IMREAD_GRAYSCALE)?;

    // highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    // highgui::imshow("window", &image1)?;
    //
    // highgui::named_window("window1", highgui::WINDOW_FULLSCREEN)?;
    // highgui::imshow("window1", &image2)?;
    //
    // let mut delta = Mat::default();
    // absdiff(&image1, &image2, &mut delta)?;
    //
    // highgui::named_window("delta", highgui::WINDOW_FULLSCREEN)?;
    // highgui::imshow("delta", &delta)?;
    //
    // highgui::named_window("threshold", highgui::WINDOW_FULLSCREEN)?;
    // let mut threshold_mat = Mat::default();
    // imgproc::threshold(
    //     &delta,
    //     &mut threshold_mat,
    //     60.0,
    //     255.0,
    //     imgproc::THRESH_BINARY,
    // )?;
    //
    // highgui::imshow("threshold", &threshold_mat)?;
    // highgui::wait_key(0)?;

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
