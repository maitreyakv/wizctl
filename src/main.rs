use anyhow::Result;
use clap::{Parser, Subcommand};
use std::net::IpAddr;
use tabled::{builder::Builder, settings::Style};
use wizctl::{client::Client, color::RGBCW};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::List => list_lights(),
        Command::Inspect { ip } => inspect_light(ip),
        Command::Set { ip, on, off, rgbcw } => set_light(ip, on, off, rgbcw),
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "List all the available lights on the local network")]
    List,
    #[clap(about = "Inspects the state and configuration of a device on the local network")]
    Inspect {
        #[clap(help = "IP address of the light to inspect")]
        ip: IpAddr,
    },
    #[clap(about = "Sets the color/state of a light")]
    Set {
        #[clap(help = "IP address of the light to set")]
        ip: IpAddr,
        #[clap(
            long,
            required = false,
            conflicts_with = "off",
            help = "Turns the light on"
        )]
        on: bool,
        #[clap(
            long,
            required = false,
            conflicts_with = "on",
            help = "Turns the light off"
        )]
        off: bool,
        #[clap(
            long,
            required = false,
            conflicts_with_all = vec!["off"],
            help = "Sets the color with an RGBCW value (e.g. \"255,250,245,0,0\")",
        )]
        rgbcw: Option<RGBCW>,
    },
}

fn list_lights() -> Result<()> {
    let client = Client::new()?;
    let mut lights = client.discover()?;
    lights.sort_by_key(|l| *l.ip());
    println!("Found {} lights on the local network", lights.len());

    let mut builder = Builder::default();
    builder.push_record(vec!["MAC", "IP"]);
    for light in lights {
        builder.push_record(vec![light.mac(), &light.ip().to_string()]);
    }
    let table = builder.build().with(Style::rounded()).to_string();
    println!("{}", table);

    Ok(())
}

fn inspect_light(_ip: &IpAddr) -> Result<()> {
    unimplemented!()
}

fn set_light(ip: &IpAddr, on: &bool, off: &bool, rgbcw_option: &Option<RGBCW>) -> Result<()> {
    let client = Client::new()?;

    if let Some(rgbcw) = rgbcw_option {
        client.set_rgbcw(ip, rgbcw)?;
        println!("Set light at {} to {}", ip, rgbcw);
        return Ok(());
    }

    if *on {
        client.turn_light_on(ip)?;
        println!("Turned on light at {}", ip);
        return Ok(());
    }

    if *off {
        client.turn_light_off(ip)?;
        println!("Turned off light at {}", ip);
        return Ok(());
    }

    println!("No change was made to the light at {}", ip);
    println!("Use `wizctl set --help` to see what you can set");
    Ok(())
}
