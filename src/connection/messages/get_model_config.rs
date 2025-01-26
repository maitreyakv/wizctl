use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetModelConfigRequest {
    method: String,
}

impl Default for GetModelConfigRequest {
    fn default() -> Self {
        Self {
            method: "getModelConfig".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GetModelConfigResponse {
    method: String,
    env: String,
    result: GetModelConfigResponseResult,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GetModelConfigResponseResult {
    ps: u8,
    #[serde(alias = "pwmFreq")]
    pwm_freq: u16,
    #[serde(alias = "pwmRange")]
    pwm_range: [u8; 2],
    wcr: u8,
    nowc: u8,
    #[serde(alias = "cctRange")]
    cct_range: [u16; 4],
    #[serde(alias = "renderFactor")]
    render_factor: [u8; 10],
    #[serde(alias = "srvIface")]
    drv_iface: Option<u8>,
}
