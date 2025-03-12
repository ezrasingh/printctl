use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Submit print job
    Print {
        /// Specify path to gcode file
        #[arg(short, long)]
        gcode: PathBuf,

        /// Specify a location for the document store (default is current directory)
        #[arg(env = "HTTP_URL")]
        http_endpoint: Option<String>,
    },
}
