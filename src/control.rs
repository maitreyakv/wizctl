use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::str;
use thiserror::Error;

use crate::{
    messages::{
        get_pilot::{GetPilotRequest, GetPilotResponse},
        get_system_config::{GetSystemConfigRequest, GetSystemConfigResponse},
        set_pilot::{SetPilotRequest, SetPilotResponse},
    },
    network::UdpClient,
};

#[derive(Error, Debug)]
#[error("Could not perform control operation!")]
pub struct ControlError;

fn send_request_and_parse_response<T>(
    ip: &Ipv4Addr,
    request: impl Serialize,
) -> error_stack::Result<T, ControlError>
where
    T: for<'a> Deserialize<'a>,
{
    let request_data = serde_json::to_vec(&request).change_context(ControlError)?;

    let client = UdpClient::new(false).change_context(ControlError)?;
    let datagram = client
        .send_udp_and_receive_response(request_data, *ip)
        .change_context(ControlError)?;

    let decoded_data = str::from_utf8(datagram.data())
        .change_context(ControlError)
        .attach_printable("Could not decode datagram data to JSON string!")?;
    serde_json::from_str(decoded_data)
        .change_context(ControlError)
        .attach_printable("Could not deserialize response data!")
}

pub fn set_pilot(ip: &Ipv4Addr, request: SetPilotRequest) -> error_stack::Result<(), ControlError> {
    let response: SetPilotResponse = send_request_and_parse_response(ip, request)?;
    let result = response.result();
    if !result.success() {
        return error_stack::Result::Err(ControlError.into())
            .attach_printable("Received \"success = false\" from light!");
    }

    Ok(())
}

pub fn describe(ip: &Ipv4Addr) -> error_stack::Result<(), ControlError> {
    let request = GetPilotRequest::default();
    let response: GetPilotResponse = send_request_and_parse_response(ip, request)?;

    dbg!(response);

    let request = GetSystemConfigRequest::default();
    let response: GetSystemConfigResponse = send_request_and_parse_response(ip, request)?;

    dbg!(response);

    Ok(())
}
