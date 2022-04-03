use opencv::core::Mat;
use opencv::{prelude::*, *};

use crate::helpers::*;

pub struct TemporalDifference {
    frame1: Option<Mat>,
    frame2: Option<Mat>,
    settings: Settings,
}

#[derive(Copy, Clone)]
pub struct Settings {
    pub blur_size: i32,
    pub thresh: f64,
}

impl TemporalDifference {
    pub fn new(settings: Settings) -> TemporalDifference {
        TemporalDifference {
            frame1: None,
            frame2: None,
            settings,
        }
    }

    pub fn process(&mut self, frame: &Mat) -> opencv::Result<Option<Mat>> {
        let mut output: Option<Mat> = None;

        if self.frame1.is_some() && self.frame2.is_some() {
            output = Some(self.calculate_difference(frame)?);
        }
        self.frame2 = self.frame1.clone();
        self.frame1 = Some(frame.clone());

        Ok(output)
    }

    fn calculate_difference(&self, frame: &Mat) -> Result<Mat> {
        let d1 = frame_difference(frame, self.frame1.as_ref().unwrap())?;
        let d2 = frame_difference(self.frame1.as_ref().unwrap(), self.frame2.as_ref().unwrap())?;

        let mut output = bitwise_or(&d1, &d2)?;
        output = median_blur(&output, self.settings.blur_size)?;
        output = binary_threshold(&output, self.settings.thresh, 255.)?;
        Ok(output)
    }
}
