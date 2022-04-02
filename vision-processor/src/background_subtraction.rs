use opencv::core::Mat;
use opencv::{prelude::*, *};

use crate::helpers::*;

pub struct BackgroundSubtraction {
    bg: Option<Mat>,
}

#[derive(Copy, Clone)]
pub struct Settings {
    pub thresh: f64,
    pub blur_size: i32,
}

impl BackgroundSubtraction {
    pub fn new() -> BackgroundSubtraction {
        BackgroundSubtraction { bg: None }
    }

    pub fn calculate_difference(
        &self,
        frame: &Mat,
        settings: Settings,
    ) -> opencv::Result<Option<Mat>> {
        if self.bg.is_none() {
            return Ok(None);
        }
        let mut output = frame_difference(frame, self.bg.as_ref().unwrap())?;
        output = median_blur(&output, settings.blur_size)?;
        output = binary_threshold(&output, settings.thresh, 255.)?;
        Ok(Some(output))
    }

    pub fn update_background(
        &mut self,
        frame: &Mat,
        mask: &Mat,
        settings: Settings,
    ) -> opencv::Result<()> {
        if self.bg.is_none() {
            self.bg = Some(frame.clone());
        } else {
            // create update image with background pixels from new image,
            // and use the existing image for the rest of the pixels

            // since we don't want to update the background with foreground pixels
            // we apply a mask to the image to select only background pixels
            let masked_bg = apply_mask(frame, &bitwise_not(mask)?)?;

            // if foreground don't update
            // if background alpha * foreground + (1 - alpha) * background

            // mask is zero for background and 255 for foreground
            // invert the mask so it is a background mask
        }

        Ok(())
    }
}
