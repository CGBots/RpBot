//! Module containing business logic for ping command.
use std::time::SystemTime;

pub struct PingCommandData{
    /// command creation timestamp in millisecond
    #[allow(dead_code)]
    timestamp: u128,
    /// delay calculated in millisecond
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