use opencv::core::Mat;
use opencv::*;

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

pub fn apply_mask(x: &Mat, mask: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::bitwise_and(x, x, &mut output, mask)?;
    Ok(output)
}

pub fn bitwise_not(x1: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::bitwise_not(x1, &mut output, &Mat::default())?;
    Ok(output)
}

pub fn add(x1: &Mat, x2: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::add(x1, x2, &mut output, &Mat::default(), -1)?;
    Ok(output)
}

pub fn weighted_sum(x1: &Mat, x2: &Mat, alpha: f64) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::add_weighted(x1, alpha, x2, 1.0 - alpha, 0., &mut output, -1)?;
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

pub fn resize(x: &Mat, size: [i32; 2]) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::imgproc::resize(
        x,
        &mut output,
        opencv::core::Size::new(size[0], size[1]),
        0.,
        0.,
        opencv::imgproc::INTER_AREA,
    )?;
    Ok(output)
}

pub fn to_gray(x: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::imgproc::cvt_color(x, &mut output, imgproc::COLOR_RGB2GRAY, 1)?;
    Ok(output)
}
