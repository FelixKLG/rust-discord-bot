use error_stack::{Context, Report, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

static CONFIG_FILE_NAME: &'static str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub discord_token: String,
    pub guilds: Vec<i64>,
}

#[derive(Debug)]
pub struct ConfigFileError;

impl fmt::Display for ConfigFileError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error validating inner values")
    }
}

impl Context for ConfigFileError {}

impl ConfigFile {
    pub fn new(token: String, guilds: Vec<i64>) -> Self {
        Self {
            discord_token: token,
            guilds,
        }
    }

    pub fn write(&self) -> Result<(), ConfigFileError> {
        let encoded_toml = toml::to_string(self).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to encode config")
                .change_context(ConfigFileError)
        })?;

        fs::write(format!("./{}", CONFIG_FILE_NAME), encoded_toml).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to write encoded config file")
                .change_context(ConfigFileError)
        })?;

        Ok(())
    }

    pub fn read() -> Result<ConfigFile, ConfigFileError> {
        let config_file = fs::read(format!("./{}", CONFIG_FILE_NAME)).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to read config file (./config.toml)")
                .change_context(ConfigFileError)
        })?;

        let encoded_config_file = String::from_utf8(config_file).map_err(|e| {
            Report::from(e)
                .attach_printable(
                    "Failed to encode config file to UTF-8 (Ensure their is no unicode)",
                )
                .change_context(ConfigFileError)
        })?;

        let decoded_config: ConfigFile =
            toml::from_str(encoded_config_file.as_str()).map_err(|e| {
                Report::from(e)
                    .attach_printable("Failed to decode config file. Its likely invalid.")
                    .change_context(ConfigFileError)
            })?;

        Ok(decoded_config)
    }
}
