use opencv::core::Mat;
use opencv::platform_types::size_t;
use opencv::{prelude::*, *};

pub struct TemporalDifference {
    frame1: Option<Mat>,
    frame2: Option<Mat>,
}

#[derive(Copy, Clone)]
pub struct Settings {
    pub blur_size: i32,
    pub thresh: f64,
}

impl TemporalDifference {
    pub fn new() -> TemporalDifference {
        TemporalDifference {
            frame1: None,
            frame2: None,
        }
    }

    pub fn process(&mut self, frame: &Mat, settings: Settings) -> opencv::Result<Option<Mat>> {
        let mut output: Option<Mat> = None;

        if self.frame1.is_some() && self.frame2.is_some() {
            let d1 = frame_difference(frame, self.frame1.as_ref().unwrap())?;
            let d2 =
                frame_difference(self.frame1.as_ref().unwrap(), self.frame2.as_ref().unwrap())?;

            let mut processing_frame = bitwise_or(&d1, &d2)?;
            processing_frame = median_blur(&processing_frame, settings.blur_size)?;
            processing_frame = binary_threshold(&processing_frame, settings.thresh, 255.)?;
            output = Some(processing_frame);
        }
        self.frame2 = self.frame1.clone();
        self.frame1 = Some(frame.clone());

        Ok(output)
    }
}

fn frame_difference(x1: &Mat, x2: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::absdiff(x1, x2, &mut output)?;
    Ok(output)
}

fn bitwise_or(x1: &Mat, x2: &Mat) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::core::bitwise_or(x1, x2, &mut output, &Mat::default())?;
    Ok(output)
}

fn median_blur(x: &Mat, kernel_size: i32) -> Result<Mat> {
    let mut output = Mat::default();
    opencv::imgproc::median_blur(x, &mut output, kernel_size)?;
    Ok(output)
}

fn binary_threshold(x: &Mat, thresh: f64, max_val: f64) -> Result<Mat> {
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
