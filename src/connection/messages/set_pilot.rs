use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::SetResponse;

//use crate::color::RGBCW;

const METHOD: &str = "setPilot";

#[derive(Debug, Serialize)]
pub struct SetPilotRequest {
    method: String,
    params: SetPilotRequestParams,
}

impl SetPilotRequest {
    //pub fn rgbcw(rgbcw: &RGBCW) -> Self {
    //    Self {
    //        method: METHOD.to_string(),
    //        params: SetPilotRequestParams {
    //            r: Some(*rgbcw.r()),
    //            g: Some(*rgbcw.g()),
    //            b: Some(*rgbcw.b()),
    //            c: Some(*rgbcw.c()),
    //            w: Some(*rgbcw.w()),
    //            ..Default::default()
    //        },
    //    }
    //}

    pub fn on() -> Self {
        Self {
            method: METHOD.to_string(),
            params: SetPilotRequestParams {
                state: Some(true),
                ..Default::default()
            },
        }
    }

    pub fn off() -> Self {
        Self {
            method: METHOD.to_string(),
            params: SetPilotRequestParams {
                state: Some(false),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Default, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    dimming: Option<u8>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct SetPilotResponse {
    method: String,
    env: String,
    result: SetPilotResponseResult,
}

impl SetResponse for SetPilotResponse {
    fn success(&self) -> bool {
        *self.result().success()
    }
}

#[derive(Debug, Deserialize, Getters)]
pub struct SetPilotResponseResult {
    success: bool,
}
