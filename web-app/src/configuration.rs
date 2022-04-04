use config::Config;
use secrecy::Secret;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub data_directory: String,
    pub secret_key: Secret<String>,
    pub redis_uri: String,
    pub username: String,
    pub password: Secret<String>,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("config-web-server"))
        .build()?;
    config.try_deserialize()
}
