use anyhow::Result;
use clap::Parser;

mod execute;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Execute(execute::Args),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Execute(args) => execute::execute(args),
    }
}
