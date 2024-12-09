use anyhow::Result;
use clap::{Parser, Subcommand};
use std::net::IpAddr;
use tabled::{builder::Builder, settings::Style};
use wizctl::client::Client;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::List => list_lights(),
        Command::Set { ip, on, off } => set_light(ip, on, off),
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
    },
}

fn list_lights() -> Result<()> {
    let client = Client::default();
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

fn set_light(ip: &IpAddr, on: &bool, off: &bool) -> Result<()> {
    let client = Client::default();

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
    Ok(())
}
