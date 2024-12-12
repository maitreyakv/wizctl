use anyhow::Result;
use clap::{Parser, Subcommand};
use std::net::Ipv4Addr;
use wizctl::client::Client;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::List => list_lights(),
        Command::Set { ip } => set_light(ip),
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
        ip: Ipv4Addr,
    },
}

fn list_lights() -> Result<()> {
    let client = Client::new();

    let mut lights = client.discover()?;
    lights.sort_by_key(|l| *l.ip());

    println!("Found {} lights on the local network", lights.len());
    for light in lights {
        println!("{:>15} - {}", light.ip(), light.mac());
    }

    Ok(())
}

fn set_light(ip: &Ipv4Addr) -> Result<()> {
    dbg!(ip);
    Ok(())
}
