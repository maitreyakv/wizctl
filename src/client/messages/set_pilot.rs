use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SetPilotRequest {
    method: String,
    params: SetPilotRequestParams,
}

impl SetPilotRequest {
    pub fn on() -> Self {
        Self {
            method: "setPilot".to_string(),
            params: SetPilotRequestParams {
                state: Some(true),
                ..Default::default()
            },
        }
    }

    pub fn off() -> Self {
        Self {
            method: "setPilot".to_string(),
            params: SetPilotRequestParams {
                state: Some(false),
                ..Default::default()
            },
        }
    }
}

#[derive(Default, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct SetPilotResponse {
    method: String,
    env: String,
    result: SetPilotResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct SetPilotResponseResult {
    success: bool,
}
