use crate::*;
use config::Config;

#[derive(serde::Deserialize, Debug)]
pub struct Configuration {
    pub recorder: recorder::Settings,
    pub mailer: mailer::Settings,
    pub camera: camera::Settings,
    pub motion_detector: motion_detector::Settings,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("config-cam-server"))
        .build()?;
    config.try_deserialize()
}
