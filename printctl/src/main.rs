mod error;
mod prelude;

mod cli;
mod command;
mod dashboard;

use crate::prelude::*;

use cli::Cli;
use command::CommandRunner;

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;

    let cli = Cli::parse();
    let runner: CommandRunner = cli
        .command
        .expect("Invalid command. Please try '--help' for more information.")
        .into();

    runner.run().await
}
