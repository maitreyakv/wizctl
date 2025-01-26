use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetPowerRequest {
    method: String,
}

impl Default for GetPowerRequest {
    fn default() -> Self {
        Self {
            method: "getPower".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Getters)]
pub struct GetPowerResponse {
    method: String,
    env: String,
    result: GetPowerResponseResult,
}

#[derive(Deserialize, Getters)]
pub struct GetPowerResponseResult {
    power: u32,
}
