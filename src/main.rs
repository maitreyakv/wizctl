use std::net::Ipv4Addr;
use std::str;

use clap::{Parser, Subcommand};
use error_stack::ResultExt;
use tabled::{builder::Builder, settings::Style};
use thiserror::Error;
use wizctl::{
    color::RGBCW,
    control::{describe, perform_speedtest, set_pilot, Pilot},
    messages::get_pilot::{GetPilotRequest, GetPilotResponse},
    network::UdpClient,
};

use ::std::str::FromStr;

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
    Set {
        #[clap(help = "Local IP address of the light to turn on")]
        ip: Ipv4Addr,
        #[clap(
            help = "Turns the light on",
            long,
            required = false,
            conflicts_with = "off"
        )]
        on: bool,
        #[clap(
            help = "Turns the light off",
            long,
            required = false,
            conflicts_with = "on"
        )]
        off: bool,
        #[clap(
            help = "RGBCW color of the light, e.g. --rgb 255,255,255,0,0",
            long,
            required = false
        )]
        rgbcw: Option<String>,
        #[clap(help = "Brightness of the light [0,255]", long, required = false)]
        brightness: Option<u8>,
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
    #[error("Could not set light!")]
    FailedToSetLight,
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

fn set_light(
    ip: &Ipv4Addr,
    on: &bool,
    off: &bool,
    rgbcw: &Option<String>,
    brightness: &Option<u8>,
) -> error_stack::Result<(), AppError> {
    let mut pilot = Pilot::default();

    pilot = if *on {
        pilot.on()
    } else if *off {
        pilot.off()
    } else {
        pilot
    };

    pilot = match rgbcw {
        Some(s) => pilot.rgbcw(RGBCW::from_str(s).change_context(AppError::FailedToSetLight)?),
        None => pilot,
    };

    pilot = match brightness {
        Some(b) => pilot.brightness(*b),
        None => pilot,
    };

    set_pilot(ip, pilot).change_context(AppError::FailedToSetLight)
}

fn speedtest(ip: Ipv4Addr) -> error_stack::Result<(), AppError> {
    perform_speedtest(&ip).change_context(AppError::Speedtest)
}

fn main() -> error_stack::Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => list_lights(),
        Commands::Describe { ip } => describe_light(*ip),
        Commands::Set {
            ip,
            on,
            off,
            rgbcw,
            brightness,
        } => set_light(ip, on, off, rgbcw, brightness),
        Commands::Speedtest { ip } => speedtest(*ip),
    }
}
