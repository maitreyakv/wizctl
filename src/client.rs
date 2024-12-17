mod messages;
mod network;

use crate::{color::RGBCW, devices::Device};
use anyhow::{Context, Result};
use core::str;
use messages::{
    error::ErrorResponse,
    get_model_config::{GetModelConfigRequest, GetModelConfigResponse},
    get_power::{GetPowerRequest, GetPowerResponse},
    get_system_config::{GetSystemConfigRequest, GetSystemConfigResponse},
    set_pilot::{SetPilotRequest, SetPilotResponse},
    SetResponse,
};
use network::{broadcast_and_receive_datagrams, init_socket, send_and_receive_datagram};
use serde::{de::DeserializeOwned, Serialize};
use std::net::{IpAddr, UdpSocket};
use thiserror::Error;

const PORT: u16 = 38899;

pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket: init_socket(PORT)?,
        })
    }
}

impl Client {
    // TODO: Need more reliable discovery for lights that are off
    pub fn discover(&self) -> Result<Vec<Device>> {
        let broadcast_data = serde_json::to_vec(&GetSystemConfigRequest::default())?;
        let datagrams = broadcast_and_receive_datagrams(&self.socket, &broadcast_data, PORT)?;

        let mut devices = Vec::new();
        for datagram in datagrams {
            let response: GetSystemConfigResponse = serde_json::from_slice(datagram.data())?;
            let device = Device::new(
                datagram.source_address().ip(),
                response.result().mac().to_string(),
            );
            devices.push(device);
        }

        Ok(devices)
    }

    pub fn get_config(&self, ip: &IpAddr) -> Result<()> {
        let request = GetSystemConfigRequest::default();
        let response: GetSystemConfigResponse = self.send_get_request(ip, &request)?;
        dbg!(response);

        let request = GetModelConfigRequest::default();
        let response: GetModelConfigResponse = self.send_get_request(ip, &request)?;
        dbg!(response);

        Ok(())
    }

    pub fn get_power(&self, ip: &IpAddr) -> Result<u32> {
        let request = GetPowerRequest::default();
        Ok(*self
            .send_get_request::<GetPowerRequest, GetPowerResponse>(ip, &request)
            .map_err(|e| {
                if let Some(ClientError::ReceivedErrorResponse { ip, code, message }) =
                    e.downcast_ref::<ClientError>()
                {
                    if *code == -32601 && message == "Method not found" {
                        return ClientError::DeviceDoesNotSupportGetPower(*ip).into();
                    }
                };
                e
            })?
            .result()
            .power())
    }

    pub fn set_rgbcw(&self, ip: &IpAddr, rgbcw: &RGBCW) -> Result<()> {
        let request = SetPilotRequest::rgbcw(rgbcw);
        self.send_set_request::<SetPilotRequest, SetPilotResponse>(ip, &request)
            .with_context(|| format!("Failed request: {:?}", request))
    }

    pub fn turn_device_on(&self, ip: &IpAddr) -> Result<()> {
        let request = SetPilotRequest::on();
        self.send_set_request::<SetPilotRequest, SetPilotResponse>(ip, &request)
            .with_context(|| format!("Failed request: {:?}", request))
    }

    pub fn turn_device_off(&self, ip: &IpAddr) -> Result<()> {
        let request = SetPilotRequest::off();
        self.send_set_request::<SetPilotRequest, SetPilotResponse>(ip, &request)
            .with_context(|| format!("Failed request {:?}", request))
    }

    fn send_get_request<T, U>(&self, ip: &IpAddr, request: &T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        self.send_request_and_receive_response::<T, U>(ip, request)
    }

    fn send_set_request<T, U>(&self, ip: &IpAddr, request: &T) -> Result<()>
    where
        T: Serialize,
        U: DeserializeOwned + SetResponse,
    {
        let response = self.send_request_and_receive_response::<T, U>(ip, request)?;
        if !response.success() {
            return Err(ClientError::UnsuccessfulRequest(*ip).into());
        }
        Ok(())
    }

    fn send_request_and_receive_response<T, U>(&self, ip: &IpAddr, request: &T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let send_data = serde_json::to_vec(request)?;

        let datagram = send_and_receive_datagram(&self.socket, &send_data, ip, PORT)?;
        let response_json = str::from_utf8(datagram.data())?;
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(response_json) {
            return Err(ClientError::ReceivedErrorResponse {
                ip: *ip,
                code: *error_response.error().code(),
                message: error_response.error().message().to_string(),
            }
            .into());
        }
        serde_json::from_str(response_json).map_err(|_| {
            ClientError::UnrecognizedResponse {
                ip: *ip,
                json: response_json.to_string(),
            }
            .into()
        })
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
    #[error("Device at {0} does not support getPower!")]
    DeviceDoesNotSupportGetPower(IpAddr),
}
