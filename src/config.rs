/*!
 * Configuration module for the messaging application.
 *
 * This module provides a structure for reading and storing configuration
 * values such as the log level.
 */

use std::env;

/// Configuration structure containing application settings.
pub struct Config {
    pub log_level: String,
}

impl Config {
    /// Creates a new `Config` instance with default values.
    ///
    /// # Returns
    ///
    /// A new `Config` instance.
    pub fn new() -> Self {
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        Config { log_level }
    }
}
