/// Data model for WiZ UDP messages
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct GetPilotResponseResult {
    mac: String,
    rssi: i8,
    state: bool,
    scene_id: i8,
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
    c: Option<u8>,
    w: Option<u8>,
    dimming: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetPilotResponse {
    method: String,
    env: String,
    result: GetPilotResponseResult,
}
