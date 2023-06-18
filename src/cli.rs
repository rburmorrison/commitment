use anyhow::Result;
use clap::Parser;

mod execute;
mod install;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Run a commitment file.
    ///
    /// This is primarily used for the pre-commit hook itself.
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
