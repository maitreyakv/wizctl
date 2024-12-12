mod messages;
mod network;

use crate::devices::Light;
use anyhow::{Context, Result};
use messages::{
    get_pilot::{GetPilotRequest, GetPilotResponse},
    set_pilot::{SetPilotRequest, SetPilotResponse},
};
use network::{broadcast_udp_and_receive_responses, init_socket, send_udp_and_receive_response};
use std::net::IpAddr;
use std::str;

pub struct Client {}

// TODO: Remove client struct and use plain functions???
impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub fn discover(&self) -> Result<Vec<Light>> {
        let socket = init_socket(true, 38899)?;
        let broadcast_data = serde_json::to_vec(&GetPilotRequest::default())?;
        let datagrams = broadcast_udp_and_receive_responses(&socket, &broadcast_data, 38899)?;

        let mut lights = Vec::new();
        for datagram in datagrams {
            let response: GetPilotResponse = serde_json::from_slice(datagram.data())?;
            let light = Light::new(
                datagram.source_address().ip(),
                response.result().mac().to_string(),
            );
            lights.push(light);
        }

        Ok(lights)
    }

    pub fn turn_light_on(&self, ip: &IpAddr) -> Result<()> {
        self.send_set_pilot(ip, SetPilotRequest::on())
    }

    pub fn turn_light_off(&self, ip: &IpAddr) -> Result<()> {
        self.send_set_pilot(ip, SetPilotRequest::off())
    }

    fn send_set_pilot(&self, ip: &IpAddr, request: SetPilotRequest) -> Result<()> {
        let socket = init_socket(false, 38899)?;
        let send_data = serde_json::to_vec(&request)?;
        let datagram = send_udp_and_receive_response(&socket, &send_data, ip, 38899)?;
        let response: SetPilotResponse = serde_json::from_slice(datagram.data())?;
        // TODO: Check success
        Ok(())
    }
}
