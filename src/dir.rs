// This is free and unencumbered software released into the public domain.

use super::{SignalConfig, SignalDb};
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct SignalDir {
    pub path: PathBuf,
}

impl SignalDir {
    pub fn open(path: PathBuf) -> Self {
        Self { path }
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
