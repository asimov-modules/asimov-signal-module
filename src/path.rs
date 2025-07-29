// This is free and unencumbered software released into the public domain.

use asimov_module::getenv;
use std::path::PathBuf;

#[cfg(unix)]
/// See: https://support.signal.org/hc/en-us/articles/360007059752-Backup-and-Restore-Messages
/// See: https://github.com/signalapp/Signal-Desktop/blob/main/CONTRIBUTING.md#the-staging-environment
pub fn default_signal_path() -> PathBuf {
    let mut result: PathBuf = getenv::home().unwrap().into();
    #[cfg(target_os = "macos")]
    result.push("Library/Application Support/Signal");
    #[cfg(not(target_os = "macos"))]
    result.push(".config/Signal");
    result
}

#[cfg(windows)]
/// See: https://support.signal.org/hc/en-us/articles/360007059752-Backup-and-Restore-Messages
/// See: https://github.com/signalapp/Signal-Desktop/blob/main/CONTRIBUTING.md#the-staging-environment
pub fn default_signal_path() -> PathBuf {
    let mut result: PathBuf = getenv::appdata().unwrap().into();
    #[cfg(target_os = "windows")]
    result.push(r"Roaming\Signal");
    result
}

#[cfg(all(not(unix), not(windows)))]
pub fn default_signal_path() -> PathBuf {
    todo!("implement default_signal_path for this platform") // TODO
}
