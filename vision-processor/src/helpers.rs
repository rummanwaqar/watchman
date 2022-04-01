use opencv::core::Mat;
use opencv::{prelude::*, *};

pub fn frame_difference(x1: &Mat, x2: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::absdiff(x1, x2, &mut output)?;
    Ok(output)
}

pub fn bitwise_or(x1: &Mat, x2: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::bitwise_or(x1, x2, &mut output, &Mat::default())?;
    Ok(output)
}

pub fn median_blur(x: &Mat, kernel_size: i32) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::imgproc::median_blur(x, &mut output, kernel_size)?;
    Ok(output)
}

pub fn binary_threshold(x: &Mat, thresh: f64, max_val: f64) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::imgproc::threshold(
        x,
        &mut output,
        thresh,
        max_val,
        opencv::imgproc::THRESH_BINARY,
    )?;
    Ok(output)
}
