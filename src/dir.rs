// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use asimov_module::secrecy::SecretString;
use serde_json::{Map, Value};
use std::{borrow::ToOwned, io, path::PathBuf};

#[derive(Debug)]
pub struct SignalDir {
    pub path: PathBuf,
}

impl SignalDir {
    pub fn open(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn encrypted_key(&self) -> io::Result<Option<SecretString>> {
        let mut config = self.config_json()?;
        Ok(config
            .remove("encryptedKey")
            .and_then(|x| x.as_str().map(SecretString::from)))
    }

    pub fn config_json(&self) -> io::Result<Map<String, Value>> {
        let config_path = self.config_path();
        let config_file = std::fs::File::open(config_path)?;
        serde_json::from_reader(config_file)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
            .map(|config: Value| config.as_object().unwrap().to_owned())
    }

    pub fn config_path(&self) -> PathBuf {
        self.path.join("config.json")
    }
}
