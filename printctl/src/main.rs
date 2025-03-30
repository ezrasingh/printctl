mod error;
mod prelude;

mod cli;
mod command;
mod config;
mod dashboard;

use crate::prelude::*;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;

    Cli::parse()
        .command
        .expect("Invalid command. Please try '--help' for more information.")
        .run()
        .await
}
