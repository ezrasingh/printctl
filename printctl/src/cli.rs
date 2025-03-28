use clap::{Parser, Subcommand};
use serde::Deserialize;
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
    /// Start printctl server
    Start {
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

#[derive(Debug, Default, Deserialize)]
pub struct PrintctlConfig {
    pub server: Option<printctl_node::server::ServerConfig>,
}
