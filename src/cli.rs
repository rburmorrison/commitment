//! The entrypoint for argument parsing and subcommand execution.

use anyhow::Result;
use clap::Parser;

mod execute;
mod install;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Run a commitment file.
    ///
    /// This can be used to test the commitment file before installing it.
    Execute(execute::Args),

    /// Install the pre-commit hook to run the commitment file.
    Install(install::Args),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Execute(args) => execute::execute(args),
        Command::Install(args) => install::execute(args),
    }
}
