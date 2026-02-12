mod error;
mod ports;

use error::Error;
use ports::Ports;

use clap::{Parser, ValueEnum};


// doas port install world/rxfetch@1.0
// doas port remove world/rxfetch@1.0


#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
    /// Install one or more ports
    Install,
    /// Remove one or more ports from the system
    Remove,
}

#[derive(Parser)]
pub struct Args {
    action: Action,
    ports: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = ports::install(args.ports) {
        eprintln!("error: {}", err);
    }
}


