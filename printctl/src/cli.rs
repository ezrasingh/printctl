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
        #[arg(long, action)]
        web: bool,

        #[arg(short, long, env = "PRINTCTL_UI_PORT")]
        port: Option<u16>,
    },
    /// List available connections
    List {
        /// List connected devices
        #[arg(short, long, action)]
        device: bool,

        /// List connected nodes
        #[arg(short, long, action)]
        node: bool,
    },
    /// Start printctl node
    Node {
        /// The address the server will bind to
        #[arg(short, long, env = "PRINTCTL_NODE_ADDR")]
        addr: Option<std::net::IpAddr>,

        /// The port the server will listen on
        #[arg(short, long, env = "PRINTCTL_NODE_PORT")]
        port: Option<u16>,

        /// Specify a config file
        #[arg(short, long = "config", env = "PRINTCTL_CONFIG")]
        config_path: Option<PathBuf>,
    },
}
