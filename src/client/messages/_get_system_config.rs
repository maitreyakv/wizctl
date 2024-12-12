use derive_getters::Getters;
use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};

use crate::messages::mac_addr_6_from_str;

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

#[derive(Deserialize, Getters, Debug)]
pub struct GetSystemConfigResponseResult {
    #[serde(deserialize_with = "mac_addr_6_from_str")]
    mac: MacAddr6,
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

#[derive(Deserialize, Debug, Getters)]
pub struct GetSystemConfigResponse {
    method: String,
    env: String,
    result: GetSystemConfigResponseResult,
}
