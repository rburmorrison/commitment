#![warn(clippy::pedantic, clippy::nursery)]

use anyhow::Result;
use defs::APP_DATA_DIR;

mod cli;
mod config;
mod defs;
mod interpreter;
mod scriptgen;
mod temp;

fn main() -> Result<()> {
    std::fs::create_dir_all(&*APP_DATA_DIR)?;
    cli::execute()?;
    Ok(())
}
