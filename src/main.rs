/// CLI tool for controlling WiZ Connected devices
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str;

use clap::{Parser, Subcommand};
use error_stack::ResultExt;
use tabled::{builder::Builder, settings::Style};
use thiserror::Error;
use wizctl::message::SetPilotResponse;
use wizctl::{
    message::{GetPilotRequest, GetPilotResponse, SetPilotRequest},
    network::{broadcast_udp_and_receive_responses, send_udp_and_receive_response},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Describe {
        #[clap(help = "Local IP address of the light to describe")]
        ip: Ipv4Addr,
    },
    On {
        #[clap(help = "Local IP address of the light to turn on")]
        ip: Ipv4Addr,
    },
    Off {
        #[clap(help = "Local IP address of the light to turn off")]
        ip: Ipv4Addr,
    },
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Could not list lights!")]
    ListLights,
    //#[error("Could not describe light!")]
    //DescribeLight,
    #[error("Could not turn on light!")]
    TurnLightOn,
    #[error("Could not turn off light!")]
    TurnLightOff,
}

fn list_lights() -> error_stack::Result<(), AppError> {
    let request = GetPilotRequest::default();
    let request_data = serde_json::to_vec(&request)
        .attach_printable("Could not serialize getPilot request to JSON!")
        .change_context(AppError::ListLights)?;

    let mut datagrams = broadcast_udp_and_receive_responses(request_data, 38899)
        .attach_printable("Could not broadcast getPilot UDP request and/or receive responses!")
        .change_context(AppError::ListLights)?;
    datagrams.sort_by_key(|d| *d.source_address());

    let mut table_builder = Builder::with_capacity(datagrams.len(), 6);
    table_builder.push_record(vec![
        "IP address",
        "MAC address",
        "signal",
        "state",
        "scene",
        "brightness",
        "rgbcw",
    ]);

    for datagram in datagrams {
        let decoded_data = String::from_utf8(datagram.data().clone())
            .attach_printable("Could not decode datagram data to JSON string!")
            .change_context(AppError::ListLights)?;
        let response: GetPilotResponse = serde_json::from_str(&decoded_data)
            .attach_printable("Could not deserialize getPilot response data!")
            .attach_printable(decoded_data)
            .change_context(AppError::ListLights)?;

        let result = response.result();

        table_builder.push_record(vec![
            datagram.source_address().ip().to_string(),
            result.mac().to_string(),
            result.signal_strength(),
            if *result.state() {
                "on".to_string()
            } else {
                "off".to_string()
            },
            result.scene_id().to_string(),
            result.dimming().to_string(),
            format!(
                "({},{},{},{},{})",
                result.r().map(|v| v.to_string()).unwrap_or_default(),
                result.g().map(|v| v.to_string()).unwrap_or_default(),
                result.b().map(|v| v.to_string()).unwrap_or_default(),
                result.c().map(|v| v.to_string()).unwrap_or_default(),
                result.w().map(|v| v.to_string()).unwrap_or_default(),
            )
            .replace("(,,,,)", ""),
        ]);
    }

    let table = table_builder.build().with(Style::rounded()).to_string();
    println!("{}", table);

    Ok(())
}

fn describe_light(_ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    unimplemented!();
}

fn turn_on_light(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    let request = SetPilotRequest::on();
    let request_data = serde_json::to_vec(&request)
        .attach_printable("Could not serialize setPilot request to JSON!")
        .change_context(AppError::TurnLightOn)?;

    let datagram = send_udp_and_receive_response(request_data, SocketAddrV4::new(ip, 38899))
        .change_context(AppError::TurnLightOn)?;

    let decoded_data = str::from_utf8(datagram.data())
        .attach_printable("Could not decode datagram data to JSON string!")
        .change_context(AppError::TurnLightOn)?;
    let response: SetPilotResponse = serde_json::from_str(decoded_data)
        .attach_printable("Could not deserialize setPilot response data!")
        .change_context(AppError::TurnLightOn)?;
    let result = response.result();
    if !result.success() {
        return error_stack::Result::Err(AppError::TurnLightOn.into())
            .attach_printable("Received \"success = false\" from light!");
    }

    println!("turned on light at {}", ip);
    Ok(())
}

fn turn_off_light(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    let request = SetPilotRequest::off();
    let request_data = serde_json::to_vec(&request)
        .attach_printable("Could not serialize setPilot request to JSON!")
        .change_context(AppError::TurnLightOff)?;

    let datagram = send_udp_and_receive_response(request_data, SocketAddrV4::new(ip, 38899))
        .change_context(AppError::TurnLightOff)?;

    let decoded_data = str::from_utf8(datagram.data())
        .attach_printable("Could not decode datagram data to JSON string!")
        .change_context(AppError::TurnLightOff)?;
    let response: SetPilotResponse = serde_json::from_str(decoded_data)
        .attach_printable("Could not deserialize setPilot response data!")
        .change_context(AppError::TurnLightOff)?;
    let result = response.result();
    if !result.success() {
        return error_stack::Result::Err(AppError::TurnLightOff.into())
            .attach_printable("Received \"success = false\" from light!");
    }

    Ok(())
}

fn main() -> error_stack::Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => list_lights(),
        Commands::Describe { ip } => describe_light(*ip),
        Commands::On { ip } => turn_on_light(*ip),
        Commands::Off { ip } => turn_off_light(*ip),
    }
}
