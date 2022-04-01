use opencv::core::Mat;
use opencv::{prelude::*, *};

use crate::helpers::*;

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
