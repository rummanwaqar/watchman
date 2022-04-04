use crate::*;
use opencv::prelude::*;
use opencv::Result;

pub struct MotionDetector {
    temporal_difference: temporal_difference::TemporalDifference,
    background_difference: background_subtraction::BackgroundSubtraction,
    settings: Settings,
}

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    motion_threshold: f64,
    pub temporal_diff: temporal_difference::Settings,
    pub background_diff: background_subtraction::Settings,
}

impl MotionDetector {
    pub fn new(settings: Settings) -> Self {
        MotionDetector {
            temporal_difference: temporal_difference::TemporalDifference::new(
                settings.temporal_diff,
            ),
            background_difference: background_subtraction::BackgroundSubtraction::new(
                settings.background_diff,
            ),
            settings,
        }
    }

    pub fn process(&mut self, frame: &Mat) -> Result<(Mat, bool)> {
        let gray = helpers::to_gray(frame)?;
        let temporal_output = self.temporal_difference.process(&gray)?;
        let background_output = self.background_difference.process(&gray)?;

        if temporal_output.is_some() && background_output.is_some() {
            let background_output = background_output.as_ref().unwrap();
            let temporal_output = temporal_output.as_ref().unwrap();
            let mask = helpers::bitwise_or(background_output, temporal_output)?;
            let motion_ratio = calculate_motion_from_mask(&mask)?;
            println!("{}", motion_ratio);
            return Ok((mask, motion_ratio > self.settings.motion_threshold));
        }
        Ok((Mat::default(), false))
    }
}

fn calculate_motion_from_mask(mask: &Mat) -> Result<f64> {
    let lit_pixels = opencv::core::sum_elems(&mask)?[0] / 255.0;
    Ok(lit_pixels / (mask.cols() * mask.rows()) as f64)
}
