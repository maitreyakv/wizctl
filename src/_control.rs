use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use std::str;
use std::time::Duration;
use std::{net::Ipv4Addr, time::Instant};
use thiserror::Error;

use crate::{
    color::RGBCW,
    messages::{
        get_pilot::{GetPilotRequest, GetPilotResponse},
        get_system_config::{GetSystemConfigRequest, GetSystemConfigResponse},
        set_pilot::{SetPilotRequest, SetPilotRequestParams, SetPilotResponse},
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

#[derive(Debug, Default)]
pub struct Pilot {
    state: Option<bool>,
    rgbcw: Option<RGBCW>,
    dimming: Option<u8>,
}

impl Pilot {
    pub fn on(mut self) -> Self {
        self.state = Some(true);
        self
    }

    pub fn off(mut self) -> Self {
        self.state = Some(false);
        self
    }

    pub fn rgbcw(mut self, rgbcw: RGBCW) -> Self {
        self.rgbcw = Some(rgbcw);
        self
    }

    pub fn brightness(mut self, brightness: u8) -> Self {
        self.dimming = Some(brightness);
        self
    }

    fn build_request(&self) -> SetPilotRequest {
        let mut params = SetPilotRequestParams::default().state(self.state);

        params = match &self.rgbcw {
            Some(rgbcw) => params
                .r(Some(*rgbcw.r()))
                .g(Some(*rgbcw.g()))
                .b(Some(*rgbcw.b()))
                .c(Some(*rgbcw.c()))
                .w(Some(*rgbcw.w())),
            None => params,
        };

        SetPilotRequest::default().params(params)
    }
}

pub fn set_pilot(ip: &Ipv4Addr, pilot: Pilot) -> error_stack::Result<(), ControlError> {
    let request = pilot.build_request();
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

pub fn perform_speedtest(ip: &Ipv4Addr) -> error_stack::Result<(), ControlError> {
    let client = UdpClient::new(false).change_context(ControlError)?;

    let mut total_messages_sent: usize = 0;
    let mut total_bytes_sent: usize = 0;
    let mut total_bytes_received: usize = 0;

    let test_duration = Duration::from_secs(10);
    let start = Instant::now();
    while start.elapsed() < test_duration {
        let request = Pilot::default().on().rgbcw(RGBCW::white()).build_request();
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
        total_bytes_received as f32 / test_duration.as_secs_f32() / 1_000.0
    );

    Ok(())
}
