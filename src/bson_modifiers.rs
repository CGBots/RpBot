use std::str::FromStr;
use serde::{Deserialize, Deserializer};

#[allow(dead_code)]
fn to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error> 
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    u64::from_str(&s).map_err(serde::de::Error::custom)
}