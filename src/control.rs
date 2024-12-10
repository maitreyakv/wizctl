use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::str;
use std::time::Duration;
use std::{net::Ipv4Addr, time::Instant};
use thiserror::Error;

use crate::{
    color::RGBCW,
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
        .send_udp_and_receive_response(&request_data, ip)
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

const TWO_PI: f32 = 2.0 * PI;

pub fn perform_speedtest(ip: &Ipv4Addr) -> error_stack::Result<(), ControlError> {
    let client = UdpClient::new(false).change_context(ControlError)?;

    let mut total_messages_sent: usize = 0;
    let mut total_bytes_sent: usize = 0;
    let mut total_bytes_received: usize = 0;

    let test_duration = Duration::from_secs(60);
    let start = Instant::now();
    while start.elapsed() < test_duration {
        let t = start.elapsed().as_secs_f32();
        let b = 127.0 * (TWO_PI * t / 10.0).cos() + 127.0;

        let request = SetPilotRequest::color(&RGBCW::white()).brightness(b as u8);
        let request_data = serde_json::to_vec(&request).change_context(ControlError)?;
        let datagram = client
            .send_udp_and_receive_response(&request_data, ip)
            .change_context(ControlError)?;

        total_messages_sent += 1;
        total_bytes_sent += request_data.len();
        total_bytes_received += datagram.data().len();
    }

    println!(
        "Sent {} round trip setPilot requests in {:?} ({} Hz)",
        total_messages_sent,
        test_duration,
        total_messages_sent as f32 / test_duration.as_secs_f32()
    );
    println!(
        "Sent {} bytes of request data ({:.2} kB/s)",
        total_bytes_sent,
        total_bytes_sent as f32 / test_duration.as_secs_f32() / 1_000.0
    );
    println!(
        "Received {} bytes of response data ({:.2} kB/s)",
        total_bytes_received,
        total_bytes_received as f32 / test_duration.as_secs_f32() / 1000.0
    );

    Ok(())
}
