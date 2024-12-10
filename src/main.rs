use std::net::Ipv4Addr;
use std::str;

use clap::{Parser, Subcommand};
use error_stack::ResultExt;
use tabled::{builder::Builder, settings::Style};
use thiserror::Error;
use wizctl::{
    color::RGBCW,
    control::{describe, perform_speedtest, set_pilot},
    messages::{
        get_pilot::{GetPilotRequest, GetPilotResponse},
        set_pilot::SetPilotRequest,
    },
    network::UdpClient,
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
    Color {
        #[clap(help = "Local IP address of the light to change color")]
        ip: Ipv4Addr,
        #[clap(help = "Color provided as RGBCW, e.g. 255,255,255,255,255")]
        rgbcw: RGBCW,
    },
    Speedtest {
        #[clap(help = "Local IP address of the light to test")]
        ip: Ipv4Addr,
    },
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Could not list lights!")]
    ListLights,
    #[error("Could not describe light!")]
    DescribeLight,
    #[error("Could not turn on light!")]
    TurnLightOn,
    #[error("Could not turn off light!")]
    TurnLightOff,
    #[error("Could not change color!")]
    SetColor,
    #[error("Could not perform speed test!")]
    Speedtest,
}

fn list_lights() -> error_stack::Result<(), AppError> {
    let request = GetPilotRequest::default();
    let request_data = serde_json::to_vec(&request)
        .attach_printable("Could not serialize getPilot request to JSON!")
        .change_context(AppError::ListLights)?;

    let client = UdpClient::new(true).change_context(AppError::ListLights)?;
    let mut datagrams = client
        .broadcast_udp_and_receive_responses(request_data)
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

fn describe_light(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    describe(&ip).change_context(AppError::DescribeLight)
}

fn turn_on_light(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    let request = SetPilotRequest::on();
    set_pilot(&ip, request).change_context(AppError::TurnLightOn)?;
    println!("turned on light at {}", ip);
    Ok(())
}

fn turn_off_light(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    let request = SetPilotRequest::off();
    set_pilot(&ip, request).change_context(AppError::TurnLightOff)?;
    println!("turned off light at {}", ip);
    Ok(())
}

fn set_color(ip: Ipv4Addr, rgbcw: &RGBCW) -> error_stack::Result<(), AppError> {
    let request = SetPilotRequest::color(rgbcw);
    set_pilot(&ip, request).change_context(AppError::SetColor)?;
    println!("set color to {} for {}", rgbcw, ip);
    Ok(())
}

fn speedtest(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    perform_speedtest(&ip).change_context(AppError::Speedtest)?;

    Ok(())
}

fn main() -> error_stack::Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => list_lights(),
        Commands::Describe { ip } => describe_light(*ip),
        Commands::On { ip } => turn_on_light(*ip),
        Commands::Off { ip } => turn_off_light(*ip),
        Commands::Color { ip, rgbcw } => set_color(*ip, rgbcw),
        Commands::Speedtest { ip } => speedtest(*ip),
    }
}
