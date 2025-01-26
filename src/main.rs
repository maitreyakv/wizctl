use clap::{Parser, Subcommand};
use std::net::IpAddr;
use tabled::{builder::Builder, settings::Style};
//use wizctl::color::RGBCW;
use wizctl::devices::{Device, DeviceError};

use thiserror::Error;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    match &cli.command {
        //Command::List => list_devices(),
        //Command::Inspect { ip } => inspect_device(ip),
        Command::Set { ip, on, off } => set_device(ip, on, off),
    }
}

/// Controls WiZ Connected devices
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    //#[clap(about = "List all the available devices on the local network")]
    //List,
    //#[clap(about = "Inspects the state and configuration of a device on the local network")]
    //Inspect {
    //    #[clap(help = "IP address of the device to inspect")]
    //    ip: IpAddr,
    //},
    #[clap(about = "Sets the color/state of a device")]
    Set {
        #[clap(help = "IP address of the device to set")]
        ip: IpAddr,
        #[clap(
            long,
            required = false,
            conflicts_with = "off",
            help = "Turns the device on"
        )]
        on: bool,
        #[clap(
            long,
            required = false,
            conflicts_with = "on",
            help = "Turns the device off"
        )]
        off: bool,
        //#[clap(
        //    long,
        //    required = false,
        //    conflicts_with_all = vec!["off"],
        //    help = "Sets the color with an RGBCW value (e.g. \"255,250,245,0,0\")",
        //)]
        //rgbcw: Option<RGBCW>,
    },
}

//fn list_devices() -> Result<()> {
//    let client = Client::new()?;
//    let mut devices = client.discover()?;
//    devices.sort_by_key(|l| *l.ip());
//    println!("Found {} devices on the local network", devices.len());
//
//    let mut builder = Builder::default();
//    builder.push_record(vec!["MAC", "IP"]);
//    for device in devices {
//        builder.push_record(vec![device.mac(), &device.ip().to_string()]);
//    }
//    let table = builder.build().with(Style::rounded()).to_string();
//    println!("{}", table);
//
//    Ok(())
//}
//
//fn inspect_device(ip: &IpAddr) -> Result<()> {
//    let client = Client::new()?;
//    client.get_config(ip)?;
//
//    let power = client.get_power(ip)?;
//    dbg!(power);
//
//    Ok(())
//}

fn set_device(ip: &IpAddr, on: &bool, off: &bool) -> Result<(), CliError> {
    let device = Device::connect(ip.to_owned())?;

    //if let Some(rgbcw) = rgbcw_option {
    //    client.set_rgbcw(ip, rgbcw)?;
    //    println!("Set device {} to {}", ip, rgbcw);
    //    return Ok(());
    //}

    if *on {
        device.turn_on()?;
        println!("Turned on device at {}", ip);
        return Ok(());
    }

    if *off {
        device.turn_off()?;
        println!("Turned off device at {}", ip);
        return Ok(());
    }

    println!("No change was made to the device at {}", ip);
    println!("Use `wizctl set --help` to see what you can set");
    Ok(())
}

#[derive(Error, Debug)]
enum CliError {
    #[error("{0}")]
    DeviceError(#[from] DeviceError),
}
