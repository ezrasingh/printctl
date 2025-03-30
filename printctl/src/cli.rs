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
    /// Start user interface
    Ui {
        /// Run web interface
        #[arg(short = 'w', long = "web", action)]
        use_web: bool,

        /// The address the web server will bind to
        #[arg(short, long, env = "PRINTCTL_WEB_UI_ADDR")]
        addr: Option<std::net::IpAddr>,

        /// The port the web server will listen on
        #[arg(short, long, env = "PRINTCTL_WEB_UI_PORT")]
        port: Option<u16>,

        /// Specify a config file
        #[arg(short, long = "config", env = "PRINTCTL_CONFIG")]
        config_path: Option<PathBuf>,
    },
    /// List available connections
    List {
        /// List connected devices
        #[arg(short, long, action)]
        devices: bool,

        /// List connected machines
        #[arg(short, long, action)]
        machines: bool,
    },
    /// Submit print job
    Print {
        /// Specify machine ID
        #[arg(short, long)]
        machine_id: String,

        /// Specify device name
        #[arg(short, long)]
        device_name: String,

        /// Specify GCODE path
        #[arg(short, long)]
        gcode_path: PathBuf,
    },
    /// Connect to printer
    Stream {
        /// Specify machine ID
        #[arg(short, long)]
        machine_id: String,

        /// Specify device name
        #[arg(short, long)]
        device_name: String,

        #[arg(short, long, action)]
        log: bool,
    },
    /// Start printctl server
    Server {
        /// The address the server will bind to
        #[arg(short, long, env = "PRINTCTL_GRPC_ADDR")]
        addr: Option<std::net::IpAddr>,

        /// The port the server will listen on
        #[arg(short, long, env = "PRINTCTL_GRPC_PORT")]
        port: Option<u16>,

        /// Specify a config file
        #[arg(short, long = "config", env = "PRINTCTL_CONFIG")]
        config_path: Option<PathBuf>,
    },
}
