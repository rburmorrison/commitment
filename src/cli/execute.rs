use std::{fs::File, path::PathBuf};

use anyhow::Result;

use crate::interpreter;

#[derive(clap::Args)]
pub struct Args {
    config: PathBuf,
}

pub fn execute(args: Args) -> Result<()> {
    let file = File::open(args.config)?;
    let config = serde_yaml::from_reader(file)?;

    if let Err(err) = interpreter::interpret(&config) {
        println!();
        Err(err)
    } else {
        Ok(())
    }
}
