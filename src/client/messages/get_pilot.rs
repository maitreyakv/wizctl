use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
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

#[derive(Debug, Deserialize, Getters)]
pub struct GetPilotResponse {
    method: String,
    env: String,
    result: GetPilotResponseResult,
}

#[derive(Debug, Deserialize, Getters)]
pub struct GetPilotResponseResult {
    mac: String,
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
