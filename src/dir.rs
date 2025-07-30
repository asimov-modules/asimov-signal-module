// This is free and unencumbered software released into the public domain.

use super::{SecretKey, SignalConfig, SignalDb, decrypt_key};
use asimov_module::secrecy::ExposeSecret;
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct SignalDir {
    pub path: PathBuf,
}

impl SignalDir {
    pub fn open(path: PathBuf) -> io::Result<Self> {
        Ok(Self { path })
    }

    pub fn key(&self, password: Option<SecretKey>) -> io::Result<Option<SecretKey>> {
        let config = self.config()?;
        if let Some(key) = config.key() {
            return Ok(Some(key));
        };
        let Some(encrypted_key) = config.encrypted_key() else {
            return Ok(None);
        };
        let result = decrypt_key(password.unwrap(), encrypted_key.expose_secret())
            .map_err(|_err| io::Error::new(io::ErrorKind::Other, "invalid password"))?;
        Ok(Some(result))
    }

    pub fn config(&self) -> io::Result<SignalConfig> {
        SignalConfig::open(self.config_path())
    }

    pub fn config_path(&self) -> PathBuf {
        self.path.join("config.json")
    }

    pub fn db(&self) -> rusqlite::Result<SignalDb> {
        SignalDb::open(self.db_path())
    }

    pub fn db_path(&self) -> PathBuf {
        self.path.join("sql/db.sqlite")
    }
}
