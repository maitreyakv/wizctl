pub mod get_pilot;
pub mod get_system_config;
pub mod set_pilot;

use macaddr::MacAddr6;
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

/// The `macaddr` crate implements `Deserialize` with the `serde_std` feature,
/// but it deserializes from byte array, not from string like we'd like.
fn mac_addr_6_from_str<'de, D>(deserializer: D) -> Result<MacAddr6, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    MacAddr6::from_str(&s).map_err(de::Error::custom)
}
