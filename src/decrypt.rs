// This is free and unencumbered software released into the public domain.

use super::SecretKey;
use alloc::boxed::Box;
use asimov_module::secrecy::ExposeSecret;
use libaes::Cipher;
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;

const SALT: &[u8] = b"saltysalt";
const ROUNDS: u32 = 1003;

pub fn decrypt_key(password: SecretKey, encrypted_key: &[u8]) -> Result<SecretKey, ()> {
    match encrypted_key.strip_prefix(b"v10") {
        Some(encrypted_key) => decrypt_key_v10(password, encrypted_key),
        None => Err(()),
    }
}

pub fn decrypt_key_v10(password: SecretKey, encrypted_key: &[u8]) -> Result<SecretKey, ()> {
    // Derive the key using PBKDF2:
    let mut kek = [0u8; 16];
    pbkdf2_hmac::<Sha1>(password.expose_secret(), SALT, ROUNDS, &mut kek);

    let cipher = Cipher::new_128(&kek);

    let iv = [' ' as u8; 16];
    let decrypted_key = cipher.cbc_decrypt(&iv, encrypted_key);

    // The decrypted key is a 64-character hex string:
    assert_eq!(decrypted_key.len(), 64);

    // FIXME: decode into a 32-byte byte array:
    let ascii_key = std::str::from_utf8(&decrypted_key).unwrap();
    let decrypted_key = hex::decode(ascii_key).unwrap();

    Ok(SecretKey::new(Box::new(decrypted_key)))
}
