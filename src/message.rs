/// Data model for WiZ UDP messages
use derive_getters::Getters;
use macaddr::MacAddr6;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Serialize, Debug)]
pub struct GetPilotRequest {
    method: String,
}

impl Default for GetPilotRequest {
    fn default() -> Self {
        Self {
            method: "getPilot".to_string(),
        }
    }
}

fn mac_addr_6_from_str<'de, D>(deserializer: D) -> Result<MacAddr6, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    MacAddr6::from_str(&s).map_err(de::Error::custom)
}

#[derive(Deserialize, Debug, Getters)]
pub struct GetPilotResponseResult {
    // The `macaddr` crate implements `Deserialize` with the `serde_std` feature, but it
    // deserializes from byte array, not from string like we'd like.
    #[serde(deserialize_with = "mac_addr_6_from_str")]
    mac: MacAddr6,
    rssi: i8,
    state: bool,
    #[serde(alias = "sceneId")]
    scene_id: i8,
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
    c: Option<u8>,
    w: Option<u8>,
    dimming: u8,
}

#[derive(Deserialize, Debug, Getters)]
pub struct GetPilotResponse {
    method: String,
    env: String,
    result: GetPilotResponseResult,
}
