mod messages;
mod network;

use crate::devices::Light;
use anyhow::Result;
use messages::get_pilot::{GetPilotRequest, GetPilotResponse};
use network::{broadcast_udp_and_receive_responses, init_socket};

pub struct Client {}

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
}
