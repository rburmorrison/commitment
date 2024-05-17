use std::{fs::File, path::PathBuf};

use crossterm::style::Stylize;

use crate::interpreter::{self, InterpretResult};

#[derive(clap::Args)]
pub struct Args {
    /// The Commitment file to execute.
    #[arg(name = "FILE")]
    config: PathBuf,
}

pub fn execute(args: &Args) -> anyhow::Result<()> {
    let file = File::open(&args.config)?;
    let config = serde_yaml::from_reader(file)?;

    let result = interpreter::interpret(&config)?;
    if let InterpretResult::Failure(name) = result {
        println!();

        let message = [
            "A task failed and the commit was rejected.",
            "Please fix the errors and try again.",
        ]
        .join(" ");

        println!("{} {}", "WARNING!".black().on_red(), message.bold());
        println!();
        anyhow::bail!(r#"Task "{name}" returned a non-zero exit code"#);
    }

    Ok(())
}
