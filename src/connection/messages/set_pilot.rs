use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::SetResponse;

const METHOD: &str = "setPilot";

pub struct SetPilotRequestBuilder(SetPilotRequest);

impl SetPilotRequestBuilder {
    pub fn new() -> Self {
        Self(SetPilotRequest::default())
    }

    pub fn build(self) -> SetPilotRequest {
        self.0
    }

    pub fn state(mut self, value: bool) -> Self {
        self.0.params.state = Some(value);
        self
    }

    pub fn dimming(mut self, value: &u8) -> Self {
        self.0.params.dimming = Some(value.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct SetPilotRequest {
    method: String,
    params: SetPilotRequestParams,
}

impl Default for SetPilotRequest {
    fn default() -> Self {
        Self {
            method: METHOD.to_string(),
            params: SetPilotRequestParams::default(),
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
