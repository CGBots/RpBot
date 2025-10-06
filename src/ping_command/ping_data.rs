//! Business logic for the `/ping` command.
//!
//! This module defines the [`PingCommandData`] struct, which is responsible for
//! computing the latency between the time a slash command is created and
//! the moment the bot processes it. It is used by the `ping` command
//! to display the bot's response time in milliseconds.
//!
//! # Example
//! ```ignore
//! use crate::ping_command::ping_data::PingCommandData;
//! let timestamp = 1_696_000_000_000_u128; // timestamp in ms
//! let ping_data = PingCommandData::new(timestamp);
//! println!("Ping: {}ms", ping_data.ping);
//! ```
use std::time::SystemTime;

/// Contains information about the `/ping` command latency.
pub struct PingCommandData{
    /// Timestamp when the command was created (in milliseconds since UNIX epoch).
    #[allow(dead_code)]
    timestamp: u128,
    /// Calculated latency in milliseconds between command creation and processing.
    pub(crate) ping: u128
}

impl PingCommandData {
    /// Creates a new [`PingCommandData`] instance and calculates the ping.
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp (in milliseconds) when the command was created.
    ///
    /// # Returns
    /// A `PingCommandData` struct with the `ping` field set to the elapsed milliseconds.
    ///
    /// # Example
    /// ```ignore
    /// let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    /// let ping_data = PingCommandData::new(timestamp);
    /// println!("Latency: {}ms", ping_data.ping);
    /// ```
    pub(crate) fn new(timestamp: u128) -> Self {
        let ping = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() - timestamp;
        Self {timestamp, ping}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    /// Tests that `PingCommandData` correctly calculates zero latency
    /// when using the current timestamp.
    #[test]
    fn test_ping() {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        assert_eq!(PingCommandData::new(now).ping, 0);
    }
}