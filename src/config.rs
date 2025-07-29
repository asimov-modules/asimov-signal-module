// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use asimov_module::secrecy::SecretString;
use serde_json::{Map, Value};
use std::{borrow::ToOwned, io, path::PathBuf};

#[derive(Debug)]
pub struct SignalConfig {
    json: Map<String, Value>,
}

impl SignalConfig {
    pub fn open(path: PathBuf) -> io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let json = serde_json::from_reader(file)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
            .map(|config: Value| config.as_object().unwrap().to_owned())?;
        Ok(Self { json })
    }

    pub fn encrypted_key(&self) -> Option<SecretString> {
        self.json
            .get("encryptedKey")
            .and_then(|x| x.as_str().map(SecretString::from))
    }
}
