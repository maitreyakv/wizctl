use derive_getters::Getters;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::color::RGBCW;

#[derive(Serialize, Debug, Default, Setters)]
struct SetPilotRequestParams {
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
}

#[derive(Serialize, Debug)]
pub struct SetPilotRequest {
    method: String,
    params: SetPilotRequestParams,
}

impl SetPilotRequest {
    pub fn on() -> Self {
        Self {
            method: "setPilot".to_string(),
            params: SetPilotRequestParams::default().state(Some(true)),
        }
    }

    pub fn off() -> Self {
        Self {
            method: "setPilot".to_string(),
            params: SetPilotRequestParams::default().state(Some(false)),
        }
    }

    pub fn color(rgbcw: &RGBCW) -> Self {
        Self {
            method: "setPilot".to_string(),
            params: SetPilotRequestParams::default()
                .r(Some(*rgbcw.r()))
                .g(Some(*rgbcw.g()))
                .b(Some(*rgbcw.b()))
                .c(Some(*rgbcw.c()))
                .w(Some(*rgbcw.w())),
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
