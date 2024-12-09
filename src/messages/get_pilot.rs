use crate::messages::mac_addr_6_from_str;
use derive_getters::Getters;
use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};

use crate::network::rssi_to_signal_strength;

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

#[derive(Deserialize, Debug, Getters)]
pub struct GetPilotResponseResult {
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

impl GetPilotResponseResult {
    pub fn signal_strength(&self) -> String {
        rssi_to_signal_strength(*self.rssi())
    }
}

#[derive(Deserialize, Debug, Getters)]
pub struct GetPilotResponse {
    method: String,
    env: String,
    result: GetPilotResponseResult,
}
