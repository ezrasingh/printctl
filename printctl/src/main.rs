mod cli;
mod command;
mod error;
mod prelude;

use crate::prelude::*;

use clap::Parser;
use cli::Cli;
use command::CommandRunner;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runner: CommandRunner = cli
        .command
        .expect("Invalid command. Please try '--help' for more information.")
        .into();

    runner.run()
}
