mod messages;
mod network;

use crate::{color::RGBCW, devices::Light};
use anyhow::{Context, Result};
use core::str;
use messages::{
    error::ErrorResponse,
    get_pilot::{GetPilotRequest, GetPilotResponse},
    set_pilot::{SetPilotRequest, SetPilotResponse},
};
use network::{broadcast_udp_and_receive_responses, init_socket, send_udp_and_receive_response};
use std::net::IpAddr;
use thiserror::Error;

#[derive(Default)]
pub struct Client {}

// TODO: Remove client struct and use plain functions???
impl Client {
    // TODO: Need more reliable discovery for lights that are off
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

    pub fn set_rgbcw(&self, ip: &IpAddr, rgbcw: &RGBCW) -> Result<()> {
        let request = SetPilotRequest::rgbcw(rgbcw);
        self.send_set_pilot_request(ip, &request)
            .with_context(|| format!("Failed request: {:?}", request))
    }

    pub fn turn_light_on(&self, ip: &IpAddr) -> Result<()> {
        let request = SetPilotRequest::on();
        self.send_set_pilot_request(ip, &request)
            .with_context(|| format!("Failed request: {:?}", request))
    }

    pub fn turn_light_off(&self, ip: &IpAddr) -> Result<()> {
        let request = SetPilotRequest::off();
        self.send_set_pilot_request(ip, &request)
            .with_context(|| format!("Failed request {:?}", request))
    }

    fn send_set_pilot_request(&self, ip: &IpAddr, request: &SetPilotRequest) -> Result<()> {
        let socket = init_socket(false, 38899)?;
        let send_data = serde_json::to_vec(request)?;
        let datagram = send_udp_and_receive_response(&socket, &send_data, ip, 38899)?;

        let response_json = str::from_utf8(datagram.data())?;

        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(response_json) {
            return Err(ClientError::ReceivedErrorResponse {
                ip: *ip,
                code: *error_response.error().code(),
                message: error_response.error().message().to_string(),
            }
            .into());
        }

        let response: SetPilotResponse =
            serde_json::from_str(response_json).map_err(|_| ClientError::UnrecognizedResponse {
                ip: *ip,
                json: response_json.to_string(),
            })?;

        if !response.result().success() {
            return Err(ClientError::UnsuccessfulRequest(*ip).into());
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Received error code {code} from device {ip}: \"{message}\"!")]
    ReceivedErrorResponse {
        ip: IpAddr,
        code: isize,
        message: String,
    },
    #[error("Received unrecognized response JSON from {ip}: {json}!")]
    UnrecognizedResponse { ip: IpAddr, json: String },
    #[error("Request to {0} came back with unsuccessful response!")]
    UnsuccessfulRequest(IpAddr),
}
