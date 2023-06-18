#![warn(clippy::pedantic, clippy::nursery)]

use anyhow::{bail, Result};
use crossterm::style::Stylize;
use defs::APP_DATA_DIR;

mod cli;
mod config;
mod defs;
mod interpreter;
mod scriptgen;
mod temp;

fn main() -> Result<()> {
    std::fs::create_dir_all(&*APP_DATA_DIR)?;

    if let Err(err) = cli::execute() {
        match err.downcast_ref::<interpreter::Error>() {
            Some(interpreter::Error::TaskFailed(_)) => {
                let message = "A task failed and the commit was rejected. Please fix the errors and try again.";
                println!("{} {}", "WARNING!".black().on_red(), message.bold());
                println!();
                bail!(err);
            }
            None => bail!(err),
        }
    }

    Ok(())
}
