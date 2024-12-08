/// High level control of WiZ Connected devices
use std::net::Ipv4Addr;
use std::str;

use crate::message::SetPilotResponse;
use crate::{message::SetPilotRequest, network::UdpClient};
use error_stack::ResultExt;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Could not perform setPilot!")]
pub struct SetPilotError;

pub fn set_pilot(
    ip: &Ipv4Addr,
    request: SetPilotRequest,
) -> error_stack::Result<(), SetPilotError> {
    let request_data = serde_json::to_vec(&request).change_context(SetPilotError)?;

    let client = UdpClient::new(false).change_context(SetPilotError)?;
    let datagram = client
        .send_udp_and_receive_response(request_data, *ip)
        .change_context(SetPilotError)?;

    let decoded_data = str::from_utf8(datagram.data())
        .change_context(SetPilotError)
        .attach_printable("Could not decode datagram data to JSON string!")?;
    let response: SetPilotResponse = serde_json::from_str(decoded_data)
        .change_context(SetPilotError)
        .attach_printable("Could not deserialize setPilot response data!")?;
    let result = response.result();
    if !result.success() {
        return error_stack::Result::Err(SetPilotError.into())
            .attach_printable("Received \"success = false\" from light!");
    }

    Ok(())
}
