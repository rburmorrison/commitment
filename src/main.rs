#![warn(clippy::pedantic, clippy::nursery)]

use std::{fs::File, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use defs::APP_DATA_DIR;

mod config;
mod defs;
mod interpreter;
mod scriptgen;
mod temp;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, name = "FILE")]
    config: PathBuf,
}

fn main() -> Result<()> {
    // Create the application data directory.
    std::fs::create_dir_all(&*APP_DATA_DIR)?;

    let args = Args::parse();
    let file = File::open(args.config)?;
    let config = serde_yaml::from_reader(file)?;

    if let Err(err) = interpreter::interpret(&config) {
        println!();
        Err(err)
    } else {
        Ok(())
    }
}
