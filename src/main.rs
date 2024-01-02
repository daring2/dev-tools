use anyhow::Result;
use clap::Parser;

use dev_tools::{Cli, Commands, perform_release};

fn main() -> Result<()> {
    execute()
}

fn execute() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Release(args) => perform_release(args)
    }
}
