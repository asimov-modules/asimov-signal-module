// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;
use asimov_module::secrecy::SecretBox;

pub type SecretKey = SecretBox<Vec<u8>>;

mod classes;
pub use classes::*;

mod config;
pub use config::*;

mod db;
pub use db::*;

mod decrypt;
pub use decrypt::*;

#[cfg(feature = "std")]
mod dir;
#[cfg(feature = "std")]
pub use dir::*;

mod key;
pub use key::*;

#[cfg(feature = "std")]
mod path;
#[cfg(feature = "std")]
pub use path::*;
