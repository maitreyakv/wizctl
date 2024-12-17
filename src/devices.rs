use derive_getters::Getters;
use std::net::IpAddr;

#[derive(Debug, Getters)]
pub struct Device {
    ip: IpAddr,
    mac: String,
}

impl Device {
    pub(crate) fn new(ip: IpAddr, mac: String) -> Self {
        Self { ip, mac }
    }
}
