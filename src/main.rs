use clap::{Parser, Subcommand};
use std::net::IpAddr;
use tabled::{builder::Builder, settings::Style};
use wizctl::color::RGBCW;
use wizctl::devices::{Device, DeviceError};

use thiserror::Error;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    match &cli.command {
        Command::List => list_devices(),
        //Command::Inspect { ip } => inspect_device(ip),
        Command::Set {
            ip,
            on,
            off,
            rgbcw,
            brightness,
        } => set_device(ip, on, off, rgbcw, brightness),
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
    #[clap(about = "List all the available devices on the local network")]
    List,
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

        #[clap(
            long,
            required = false,
            conflicts_with = "off",
            help = "Sets the color with an RGBCW value (e.g. \"255,250,245,0,0\")"
        )]
        rgbcw: Option<RGBCW>,

        #[clap(
            long,
            required = false,
            conflicts_with = "off",
            help = "Sets the brightness with a values between 0 and 255"
        )]
        brightness: Option<u8>,
    },
}

fn list_devices() -> Result<(), CliError> {
    let mut devices = Device::discover()?;
    devices.sort_by_key(|l| *l.ip());
    println!("Found {} devices on the local network", devices.len());

    let mut builder = Builder::default();
    builder.push_record(vec!["MAC", "IP", "Type", "Signal"]);
    for device in devices {
        builder.push_record(vec![
            device.mac(),
            &device.ip().to_string(),
            &format!("{}", device.kind()),
            &rssi_to_signal_strength(device.get_rssi()?),
        ]);
    }
    let table = builder.build().with(Style::rounded()).to_string();
    println!("{}", table);

    Ok(())
}

fn rssi_to_signal_strength(rssi: i8) -> String {
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

//fn inspect_device(ip: &IpAddr) -> Result<()> {
//    let client = Client::new()?;
//    client.get_config(ip)?;
//
//    let power = client.get_power(ip)?;
//    dbg!(power);
//
//    Ok(())
//}

fn set_device(
    ip: &IpAddr,
    on: &bool,
    off: &bool,
    rgbcw: &Option<RGBCW>,
    brightness: &Option<u8>,
) -> Result<(), CliError> {
    let device = Device::connect(ip.to_owned())?;

    let mut builder = device.set_pilot();
    let mut messages = Vec::new();

    if *on {
        builder = builder.on();
        messages.push(format!("Turned on device at {}", ip));
    }

    if *off {
        builder = builder.off();
        messages.push(format!("Turned off device at {}", ip));
    }

    if let Some(rgbcw) = rgbcw {
        builder = builder.rgbcw(rgbcw.to_owned())?;
        messages.push(format!("Set color at {} to {}", ip, rgbcw))
    }

    if let Some(brightness) = brightness {
        builder = builder.brightness(*brightness)?;
        messages.push(format!("Set brightness at {} to {}", ip, brightness));
    }

    let device = builder.send()?;

    if messages.is_empty() {
        println!("No change was made to the device at {}", ip);
        println!("Use `wizctl set --help` to see what you can set");
    } else {
        for msg in messages {
            println!("{}", msg);
        }
    }
    Ok(())
}

#[derive(Error, Debug)]
enum CliError {
    #[error("{0}")]
    DeviceError(#[from] DeviceError),
}
