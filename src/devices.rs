use derive_getters::Getters;
use std::net::IpAddr;

#[derive(Debug, Getters)]
pub struct Light {
    ip: IpAddr,
    mac: String,
}

impl Light {
    pub fn new(ip: IpAddr, mac: String) -> Self {
        Self { ip, mac }
    }
}
