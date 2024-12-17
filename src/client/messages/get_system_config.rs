use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GetSystemConfigRequest {
    method: String,
}

impl Default for GetSystemConfigRequest {
    fn default() -> Self {
        Self {
            method: "getSystemConfig".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Getters)]
pub struct GetSystemConfigResponse {
    method: String,
    env: String,
    result: GetSystemConfigResponseResult,
}

#[allow(dead_code)]
#[derive(Deserialize, Getters, Debug)]
pub struct GetSystemConfigResponseResult {
    mac: String,
    #[serde(alias = "homeId")]
    home_id: usize,
    #[serde(alias = "roomId")]
    room_id: usize,
    rgn: String,
    #[serde(alias = "moduleName")]
    module_name: String,
    #[serde(alias = "fwVersion")]
    fw_version: String,
    #[serde(alias = "groupId")]
    group_id: usize,
    ping: usize,
}
