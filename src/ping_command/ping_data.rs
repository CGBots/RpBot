use std::time::SystemTime;

/// A structure representing the data associated with a "Ping" command.
///
/// This struct is designed to store metadata about a ping operation, 
/// including the timestamp for when the operation took place and 
/// the duration/latency of the ping.
///
/// # Fields
///
/// * `timestamp` (private):
///     A 128-bit unsigned integer representing the time at which the 
///     ping command was issued. This field is private and not directly 
///     accessible from outside the struct. It is marked with 
///     `#[allow(dead_code)]` as it may not currently be used.
///
/// * `ping` (crate-level visibility):
///     A 128-bit unsigned integer representing the measured ping value 
///     (e.g., latency). This field is visible within the current crate.
///
/// # Visibility
/// The `PingCommandData` structure is public (`pub`), but the visibility 
/// of its fields is limited:
/// - `ping` is accessible within the same crate (`pub(crate)`).
/// - `timestamp` is private and not accessible outside of this struct.
///
/// # Example
/// ```
/// use your_crate::PingCommandData;
///
/// let ping_data = PingCommandData {
///     timestamp: 1632995812356,
///     ping: 42,
/// };
///
/// // The `ping` value can be accessed within the same crate.
/// println!("Ping value: {}", ping_data.ping);
/// ```
pub struct PingCommandData{
    #[allow(dead_code)]
    timestamp: u128,
    pub(crate) ping: u128
}

impl PingCommandData {
    pub(crate) fn new(timestamp: u128) -> Self {
        let ping = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() - timestamp;
        Self {timestamp, ping}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ping() {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        assert_eq!(PingCommandData::new(now).ping, 0);
    }
}