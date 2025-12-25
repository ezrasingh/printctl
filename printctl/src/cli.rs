use clap::{Parser, Subcommand};
use std::net::IpAddr;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about = "Print Agent Controller")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Launch UI (TUI or Web)
    Ui {
        /// Run Web UI instead of TUI
        #[arg(short = 'w', long = "web")]
        use_web: bool,

        /// Web UI bind address
        #[arg(short, long, env = "PRINTCTL_WEB_ADDR")]
        addr: Option<IpAddr>,

        /// Web UI port
        #[arg(short, long, env = "PRINTCTL_WEB_PORT")]
        port: Option<u16>,
    },

    /// List detected serial devices
    ListDevices,

    /// Upload a G-code file
    UploadGcode {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Queue a print job
    QueueJob {
        #[arg(short, long)]
        printer_id: Uuid,

        #[arg(short, long)]
        gcode_id: Uuid,
    },

    /// Show job list
    ListJobs,
}
