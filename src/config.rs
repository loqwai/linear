use derive_error::Error;
use serde::Deserialize;
use std::{
    env::{self},
    fs,
};

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) api_key: String,
    pub(crate) team_name: String,
}

#[derive(Debug, Error)]
pub(crate) enum ConfigError {
    EnvVarError(env::VarError),
    FileReadError(std::io::Error),
    ParseError(toml::de::Error),
}

pub(crate) fn get_config() -> Result<Config, ConfigError> {
    let home = env::var("HOME")?;
    let file_path = format!("{}/.config/linear/config.toml", home);

    let config_str = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_str)?;

    return Ok(config);
}
