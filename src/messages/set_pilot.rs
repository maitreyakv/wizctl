use derive_getters::Getters;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default, Setters)]
pub struct SetPilotRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    g: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    c: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    w: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimming: Option<u8>,
}

#[derive(Serialize, Debug, Setters)]
pub struct SetPilotRequest {
    method: String,
    params: SetPilotRequestParams,
}

impl Default for SetPilotRequest {
    fn default() -> Self {
        Self {
            method: "setPilot".to_string(),
            params: SetPilotRequestParams::default(),
        }
    }
}

#[derive(Deserialize, Debug, Getters)]
pub struct SetPilotResponseResult {
    success: bool,
}

#[derive(Deserialize, Debug, Getters)]
pub struct SetPilotResponse {
    method: String,
    env: String,
    result: SetPilotResponseResult,
}
