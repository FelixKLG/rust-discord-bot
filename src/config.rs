use crate::hash_string::HashedString;
use bincode::{config, Decode, Encode};
use error_stack::{Context, Report, Result};
use std::{fmt, fs};

static CONFIG_FILE_NAME: &'static str = "bot.config";

#[derive(Encode, Decode, Debug)]
pub struct ConfigFile {
    discord_token: HashedString,
    guilds: Vec<i64>,
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
    pub fn new<T: Into<String>>(token: T, guilds: Vec<i64>) -> Self {
        Self {
            discord_token: HashedString::new(token.into()),
            guilds,
        }
    }

    pub fn write(&self) -> Result<(), ConfigFileError> {
        let config = config::standard();

        let vec_encoded = bincode::encode_to_vec(&self, config).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to encode config")
                .change_context(ConfigFileError)
        })?;

        fs::write(format!("./{}", CONFIG_FILE_NAME), vec_encoded).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to write encoded config file")
                .change_context(ConfigFileError)
        })?;

        Ok(())
    }

    pub fn read() -> Result<ConfigFile, ConfigFileError> {
        let config = config::standard();

        let config_file = fs::read(format!("./{}", CONFIG_FILE_NAME)).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to read config file (./bot.config)")
                .change_context(ConfigFileError)
        })?;

        let (decoded_config, _bytes): (ConfigFile, usize) =
            bincode::decode_from_slice(&config_file, config).map_err(|e| {
                Report::from(e)
                    .attach_printable("Failed to decode config file. It is likely corrupt.")
                    .change_context(ConfigFileError)
            })?;

        Ok(decoded_config)
    }
}
