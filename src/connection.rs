pub mod messages;
mod network;

use std::io;
//use crate::{color::RGBCW, devices::Device};
use core::str;
use messages::{
    error::ErrorResponse,
    get_model_config::{GetModelConfigRequest, GetModelConfigResponse},
    get_pilot::{GetPilotRequest, GetPilotResponse},
    get_power::{GetPowerRequest, GetPowerResponse},
    get_system_config::{GetSystemConfigRequest, GetSystemConfigResponse},
    set_pilot::{SetPilotRequest, SetPilotResponse},
    SetResponse,
};
use network::send_and_receive_datagram;
use network::NetworkError;
use network::{broadcast_and_receive_datagrams, init_socket};
use serde::{de::DeserializeOwned, Serialize};
use std::net::{IpAddr, UdpSocket};
use thiserror::Error;

const PORT: u16 = 38899;

pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self {
            socket: init_socket()?,
        })
    }
}

impl Connection {
    // TODO: Need more reliable discovery for lights that are off
    pub fn discover(&self) -> Result<Vec<(IpAddr, GetSystemConfigResponse)>, ConnectionError> {
        let broadcast_data = serde_json::to_vec(&GetSystemConfigRequest::default())?;
        Ok(
            broadcast_and_receive_datagrams(&self.socket, &broadcast_data, PORT)?
                .into_iter()
                .map(|datagram| {
                    serde_json::from_slice::<GetSystemConfigResponse>(datagram.data())
                        .map(|system_config| (datagram.source_address().ip(), system_config))
                })
                .collect::<Result<Vec<(IpAddr, GetSystemConfigResponse)>, _>>()?,
        )
    }

    pub fn get_system_config(
        &self,
        ip: &IpAddr,
    ) -> Result<GetSystemConfigResponse, ConnectionError> {
        let request = GetSystemConfigRequest::default();
        self.send_get_request::<GetSystemConfigRequest, GetSystemConfigResponse>(ip, &request)
    }

    //pub fn get_power(&self, ip: &IpAddr) -> Result<u32> {
    //    let request = GetPowerRequest::default();
    //    Ok(*self
    //        .send_get_request::<GetPowerRequest, GetPowerResponse>(ip, &request)
    //        .map_err(|e| {
    //            if let Some(ConnectionError::ReceivedErrorResponse { ip, code, message }) =
    //                e.downcast_ref::<ConnectionError>()
    //            {
    //                if *code == -32601 && message == "Method not found" {
    //                    return ConnectionError::DeviceDoesNotSupportGetPower(*ip).into();
    //                }
    //            };
    //            e
    //        })?
    //        .result()
    //        .power())
    //}
    //
    //pub fn set_rgbcw(&self, ip: &IpAddr, rgbcw: &RGBCW) -> Result<()> {
    //    let request = SetPilotRequest::rgbcw(rgbcw);
    //    self.send_set_request::<SetPilotRequest, SetPilotResponse>(ip, &request)
    //        .with_context(|| format!("Failed request: {:?}", request))
    //}

    pub fn get_pilot(&self, ip: &IpAddr) -> Result<GetPilotResponse, ConnectionError> {
        let request = GetPilotRequest::default();
        self.send_get_request::<GetPilotRequest, GetPilotResponse>(ip, &request)
    }

    pub fn set_pilot(&self, ip: &IpAddr, request: SetPilotRequest) -> Result<(), ConnectionError> {
        self.send_set_request::<SetPilotRequest, SetPilotResponse>(ip, &request)
    }
}

impl Connection {
    fn send_get_request<T, U>(&self, ip: &IpAddr, request: &T) -> Result<U, ConnectionError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        self.send_request_and_receive_response::<T, U>(ip, request)
    }

    fn send_set_request<T, U>(&self, ip: &IpAddr, request: &T) -> Result<(), ConnectionError>
    where
        T: Serialize,
        U: DeserializeOwned + SetResponse,
    {
        let response = self.send_request_and_receive_response::<T, U>(ip, request)?;
        if response.success() {
            Ok(())
        } else {
            Err(ConnectionError::UnsuccessfulRequest)
        }
    }

    fn send_request_and_receive_response<T, U>(
        &self,
        ip: &IpAddr,
        request: &T,
    ) -> Result<U, ConnectionError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let send_data = serde_json::to_vec(request).expect("failed to serialize request");

        let datagram = send_and_receive_datagram(&self.socket, &send_data, ip, PORT)?;
        let response_json = str::from_utf8(datagram.data())?;
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(response_json) {
            Err(ConnectionError::ErrorResponse {
                code: *error_response.error().code(),
                message: error_response.error().message().to_string(),
            })
        } else {
            Ok(serde_json::from_str(response_json)?)
        }
    }
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Received response with invalid UTF8")]
    InvalidUtf8(#[from] str::Utf8Error),
    #[error("UDP operation failed")]
    NetworkError(#[from] NetworkError),
    #[error("Received error code {code}: \"{message}\"!")]
    ErrorResponse { code: isize, message: String },
    #[error("Could not deserialize response!")]
    InvalidResponse(#[from] serde_json::Error),
    #[error("Device was not able to handle request!")]
    UnsuccessfulRequest,
    //    #[error("Device at {0} does not support getPower!")]
    //    DeviceDoesNotSupportGetPower(IpAddr),
}
