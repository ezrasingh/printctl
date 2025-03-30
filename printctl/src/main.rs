mod cli;
mod error;
mod prelude;
mod ui;

use crate::prelude::*;

fn default_config(config_path: Option<std::path::PathBuf>) -> cli::PrintctlConfig {
    use config::Config;
    config_path
        .map(|path| {
            Config::builder()
                .add_source(config::File::from(path))
                .build()
                .unwrap()
                .try_deserialize()
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;

    use crate::cli::Cli;

    let cmd = Cli::parse()
        .command
        .expect("Invalid command. Please try '--help' for more information.");
    let cwd = std::env::current_dir()?;

    match cmd {
        cli::Command::Ui {
            use_web,
            addr,
            port,
            config_path,
        } => {
            let ui_config = default_config(config_path).ui.unwrap_or_default();

            if use_web || ui_config.use_web {
                let default_port = ui_config.http_port.unwrap_or(8080);
                let port = port.unwrap_or(default_port);
                ui::web::start(port).expect("could not start web user interface");
            } else {
                ui::tui::start().expect("could not start terminal user interface");
            }
        }

        cli::Command::List { devices, machines } => {
            use printctl_node::discovery;

            println!("Starting discovery...");
            let discovery_node = discovery::Node::default().start_discovery();
            for peer in discovery_node.peers().await {
                if machines || devices {
                    println!("{:#?}", peer);
                }
            }
            println!("Ending discovery...");
            discovery_node.stop_discovery().await;
        }

        cli::Command::Server {
            addr,
            port,
            config_path,
        } => {
            use printctl_node::server::ServerConfig;

            let config = default_config(config_path);
            let (server_config, discovery_config) = (
                config.server.unwrap_or_default(),
                config.discovery.unwrap_or_default(),
            );
            let (addr, port) = (
                addr.unwrap_or(*server_config.grpc_address()),
                port.unwrap_or(*server_config.grpc_port()),
            );

            printctl_node::server::run(ServerConfig::new(addr, port), discovery_config)
                .expect("could not start server");
        }

        cli::Command::Print {
            machine_id,
            device_name,
            gcode_path,
        } => todo!(),

        cli::Command::Stream {
            machine_id,
            device_name,
            log,
        } => todo!(),
    };

    Ok(())
}
