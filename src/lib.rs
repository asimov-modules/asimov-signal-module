// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod classes;
pub use classes::*;

mod config;
pub use config::*;

mod db;
pub use db::*;

#[cfg(feature = "std")]
mod dir;
#[cfg(feature = "std")]
pub use dir::*;

#[cfg(feature = "std")]
mod path;
#[cfg(feature = "std")]
pub use path::*;
