// This is free and unencumbered software released into the public domain.

use alloc::format;
use asimov_module::secrecy::{ExposeSecret, SecretString};
use rusqlite::{Connection, OpenFlags, Result};
use std::path::Path;

#[derive(Debug)]
pub struct SignalDb {
    pub conn: Connection,
}

impl SignalDb {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_PRIVATE_CACHE,
        )?;
        Ok(Self { conn })
    }

    pub fn decrypt(&self, key: SecretString) -> Result<()> {
        self.conn
            .pragma_update(None, "key", format!("x'{}'", key.expose_secret()))
    }

    pub fn is_readable(&self) -> bool {
        self.conn
            .query_row("SELECT count(*) FROM sqlite_master LIMIT 1", [], |_row| {
                Ok(())
            })
            .is_ok()
    }
}
