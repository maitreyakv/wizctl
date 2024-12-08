/// CLI tool for controlling WiZ Connected devices
use clap::{Parser, Subcommand};
use error_stack;
use error_stack::ResultExt;
use tabled::{builder::Builder, settings::Style};
use thiserror::Error;
use wizctl::{
    message::{GetPilotRequest, GetPilotResponse},
    network::broadcast_udp,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Could not fetch status of lights")]
    ListLightsError,
}

fn rssi_to_strength_symbol(rssi: i8) -> String {
    if rssi < -70 {
        "\u{2840} ".to_string()
    } else if rssi < -60 {
        "\u{28e0} ".to_string()
    } else if rssi < -50 {
        "\u{28e0}\u{2846}".to_string()
    } else {
        "\u{28e0}\u{28fe}".to_string()
    }
}

fn list_lights() -> error_stack::Result<(), AppError> {
    let request = GetPilotRequest::default();
    let request_data = serde_json::to_vec(&request)
        .attach_printable("Could not serialize getPilot request to JSON!")
        .change_context(AppError::ListLightsError)?;

    let mut datagrams = broadcast_udp(request_data, 38899)
        .attach_printable("Could not broadcast getPilot UDP request!")
        .change_context(AppError::ListLightsError)?;
    datagrams.sort_by_key(|d| *d.source_address());

    let mut table_builder = Builder::with_capacity(datagrams.len(), 6);
    table_builder.push_record(vec![
        "IP address",
        "MAC address",
        "state",
        "scene",
        "dimming",
        "r",
        "g",
        "b",
        "c",
        "w",
        "signal",
    ]);

    for datagram in datagrams {
        let decoded_data = String::from_utf8(datagram.data().clone())
            .attach_printable("Could not decode datagram data to JSON string!")
            .change_context(AppError::ListLightsError)?;
        let response: GetPilotResponse = serde_json::from_str(&decoded_data)
            .attach_printable("Could not deserialize getPilot response data!")
            .attach_printable(decoded_data)
            .change_context(AppError::ListLightsError)?;

        let result = response.result();

        table_builder.push_record(vec![
            datagram.source_address().ip().to_string(),
            result.mac().to_string(),
            if *result.state() {
                "on".to_string()
            } else {
                "off".to_string()
            },
            result.scene_id().to_string(),
            result.dimming().to_string(),
            result.r().map(|v| v.to_string()).unwrap_or_default(),
            result.g().map(|v| v.to_string()).unwrap_or_default(),
            result.b().map(|v| v.to_string()).unwrap_or_default(),
            result.c().map(|v| v.to_string()).unwrap_or_default(),
            result.w().map(|v| v.to_string()).unwrap_or_default(),
            rssi_to_strength_symbol(*result.rssi()),
        ]);
    }

    let table = table_builder.build().with(Style::rounded()).to_string();
    println!("{}", table);

    Ok(())
}

fn main() -> error_stack::Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => list_lights(),
    }
}
