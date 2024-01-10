use base16ct::lower::encode_string;
use bincode::{Decode, Encode};
use error_stack::{Context, Report, Result};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug)]
#[allow(unused)]
pub enum HashStringError {
    DecodingError,
    ValidationError,
}

impl fmt::Display for HashStringError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodingError => fmt.write_str("Error decoding data"),
            Self::ValidationError => fmt.write_str("Error validating inner values"),
        }
    }
}

impl Context for HashStringError {}

#[derive(Encode, Decode, Debug)]
pub struct HashedString(String, String);

impl HashedString {
    #[allow(unused)]
    pub fn new<T: Into<String>>(string: T) -> Self {
        let input_string: String = string.into();

        let mut hasher = Sha256::new();

        hasher.update(input_string.as_bytes());

        let sha_hash = hasher.finalize();

        Self(input_string, encode_string(&sha_hash))
    }

    #[allow(unused)]
    pub fn check(&self) -> Result<(), HashStringError> {
        let HashedString(value, hash) = self;

        let mut hasher = Sha256::new();

        hasher.update(value.as_bytes());

        let sha_hash = hasher.finalize();

        let encoded_hash = encode_string(&sha_hash);

        if hash != &encoded_hash {
            return Err(Report::new(HashStringError::ValidationError));
        };

        Ok(())
    }

    #[allow(unused)]
    pub fn value(&self) -> Result<&String, HashStringError> {
        let HashedString(value, hash) = self;

        let mut hasher = Sha256::new();

        hasher.update(value.as_bytes());

        let sha_hash = hasher.finalize();

        let encoded_hash = encode_string(&sha_hash);

        if hash != &encoded_hash {
            return Err(Report::new(HashStringError::ValidationError));
        };

        Ok(value)
    }

    #[allow(unused)]
    pub fn value_unchecked(&self) -> &String {
        let HashedString(value, _hash) = self;

        value
    }
}
