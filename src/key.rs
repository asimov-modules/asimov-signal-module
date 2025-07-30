// This is free and unencumbered software released into the public domain.

use super::SecretKey;
use alloc::boxed::Box;
use asimov_module::{getenv, secrecy::ExposeSecret};
use hex::FromHexError;

pub fn signal_key_from_env() -> Result<Option<SecretKey>, FromHexError> {
    let Some(key_str) = getenv::var_secret("ASIMOV_SIGNAL_KEY") else {
        return Ok(None);
    };
    let key_data = hex::decode(key_str.expose_secret())?;
    Ok(Some(SecretKey::new(Box::new(key_data))))
}

pub fn signal_password_from_env() -> Result<Option<SecretKey>, FromHexError> {
    let Some(key_str) = getenv::var_secret("ASIMOV_SIGNAL_PASSWORD") else {
        return Ok(None);
    };
    let key_data = key_str.expose_secret().as_bytes().to_vec();
    Ok(Some(SecretKey::new(Box::new(key_data))))
}

pub fn signal_password_from_keychain() -> Result<Option<SecretKey>, ()> {
    todo!(r#"security find-generic-password -a "Signal Key" -s "Signal Safe Storage" -w"#) // TODO
}
