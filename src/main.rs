/// CLI tool for controlling WiZ Connected devices
use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt};
use thiserror::Error;

mod message;

use wizctl::message::GetPilotRequest;

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

fn list_lights() -> Result<(), serde_json::Error> {
    let request = GetPilotRequest::new();
    let request_data = serde_json::to_string(&request)
        .attach_printable("Could not serialize getPilot request to JSON!")
        .attach(request)?;
    dbg!(request_data);
    Ok(())
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Could not fetch status of lights")]
    ListLightsError,
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            let r = list_lights();
            r.change_context(AppError::ListLightsError)
        }
    }
}
