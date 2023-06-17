#![warn(clippy::pedantic, clippy::nursery)]

use std::{fs::File, path::PathBuf};

use anyhow::Result;
use clap::Parser;

mod config;
mod interpreter;
mod parsing;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, name = "FILE")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.config)?;
    let config = serde_yaml::from_reader(file)?;

    interpreter::interpret(&config)?;

    Ok(())
}
