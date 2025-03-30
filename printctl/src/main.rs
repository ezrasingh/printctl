mod error;
mod prelude;

mod cli;
mod config;
mod dashboard;

use crate::prelude::*;

use printctl_node::discovery;
fn setup_client_node() -> Result<discovery::Node> {
    use gethostname::gethostname;
    let name = gethostname().into_string().unwrap();
    let node = discovery::Node::new(name, &None)?;
    Ok(node)
}

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;

    use crate::cli::Cli;

    let cmd = Cli::parse()
        .command
        .expect("Invalid command. Please try '--help' for more information.");
    let cwd = std::env::current_dir()?;
    let discovery_node = setup_client_node()?;

    match cmd {
        cli::Command::Ui {
            use_web,
            addr,
            port,
            config_path,
        } => {
            if use_web {
                dashboard::web::start(port.unwrap_or(8080))
                    .expect("could not start web user interface");
            } else {
                dashboard::tui::start().expect("could not start terminal user interface");
            }
        }

        cli::Command::List { devices, machines } => {
            println!("Listening for peers...");
            let active_node = discovery_node.start_discovery();
            for _ in 0..20 {
                if machines {
                    for peer in active_node.peers().await {
                        println!("{:#?}", peer);
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            println!("Ending peer discovery...");
            active_node.stop_discovery().await;
        }

        cli::Command::Server {
            addr,
            port,
            config_path,
        } => todo!(),

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
