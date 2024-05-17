#![warn(clippy::pedantic, clippy::nursery)]

mod cli;
mod config;
mod interpreter;

fn main() -> anyhow::Result<()> {
    cli::execute()?;

    Ok(())
}
