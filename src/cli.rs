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
    Execute(execute::Args),
    Install(install::Args),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Execute(args) => execute::execute(args),
        Command::Install(args) => install::execute(args),
    }
}
