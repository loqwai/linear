use derive_error::Error;
use serde::{Deserialize, Serialize};
use std::{
    env::{self},
    fs,
};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) api_key: String,
    pub(crate) team_name: String,
    pub(crate) current_issue: Option<String>,
}

#[derive(Debug, Error)]
pub(crate) enum GetConfigError {
    EnvVarError(env::VarError),
    FileReadError(std::io::Error),
    ParseError(toml::de::Error),
}

pub(crate) fn get_config() -> Result<Config, GetConfigError> {
    let home = env::var("HOME")?;
    let file_path = format!("{}/.config/linear/config.toml", home);

    let config_str = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_str)?;

    return Ok(config);
}

#[derive(Debug, Error)]
pub(crate) enum StoreConfigError {
    EnvVarError(env::VarError),
    SerializeError(toml::ser::Error),
    FileWriteError(std::io::Error),
}

pub(crate) fn store_config(config: &Config) -> Result<(), StoreConfigError> {
    let home = env::var("HOME")?;
    let file_path = format!("{}/.config/linear/config.toml", home);

    let config_str = toml::to_string(config)?;
    fs::write(file_path, config_str)?;

    Ok(())
}
