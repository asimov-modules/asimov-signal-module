// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use asimov_module::secrecy::SecretString;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct SignalConfig {
    json: Map<String, Value>,
}

impl SignalConfig {
    #[cfg(feature = "std")]
    pub fn open(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        use std::io;
        let file = std::fs::File::open(path)?;
        let config = serde_json::from_reader(file)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
            .map(Self::from_value)?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Signal config.json must contain an object",
                )
            })?;
        Ok(config)
    }

    pub fn from_value(json: Value) -> Option<Self> {
        match json.as_object() {
            None => None,
            Some(object) => Some(Self {
                json: object.clone(),
            }),
        }
    }

    pub fn encrypted_key(&self) -> Option<SecretString> {
        self.json
            .get("encryptedKey")
            .and_then(|x| x.as_str().map(SecretString::from))
    }
}
