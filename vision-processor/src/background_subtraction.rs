use crate::helpers::*;
use opencv::core::Mat;

pub struct BackgroundSubtraction {
    bg: Option<Mat>,
    settings: Settings,
}

#[derive(Copy, Clone, serde::Deserialize, Debug)]
pub struct Settings {
    pub thresh: f64,
    pub blur_size: i32,
    pub alpha: f64,
}

impl BackgroundSubtraction {
    pub fn new(settings: Settings) -> BackgroundSubtraction {
        BackgroundSubtraction { bg: None, settings }
    }

    pub fn process(&mut self, frame: &Mat) -> opencv::Result<Option<Mat>> {
        match self.bg {
            None => {
                self.bg = Some(frame.clone());
                Ok(None)
            }
            Some(_) => {
                let mask = self.calculate_difference(frame)?;
                self.update_background(frame, &mask)?;
                Ok(Some(mask))
            }
        }
    }

    fn calculate_difference(&self, frame: &Mat) -> opencv::Result<Mat> {
        let mut output = frame_difference(frame, self.bg.as_ref().unwrap())?;
        output = median_blur(&output, self.settings.blur_size)?;
        output = binary_threshold(&output, self.settings.thresh, 255.)?;
        Ok(output)
    }

    fn update_background(&mut self, frame: &Mat, mask: &Mat) -> opencv::Result<()> {
        // create update image with background pixels from new image,
        // and use the existing image for the rest of the pixels
        let background_pixels = apply_mask(frame, &bitwise_not(mask)?)?;
        let foreground_pixels = apply_mask(self.bg.as_ref().unwrap(), mask)?;
        let update_image = add(&background_pixels, &foreground_pixels)?;

        // update background with weighted sum of old and new background
        self.bg = Some(weighted_sum(
            &update_image,
            self.bg.as_ref().unwrap(),
            self.settings.alpha,
        )?);
        Ok(())
    }
}
