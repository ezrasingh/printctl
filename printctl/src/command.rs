use crate::prelude::*;

use crate::cli;
use crate::dashboard;

use printctl_node::discovery;

fn setup_client_node() -> Result<discovery::Node> {
    use gethostname::gethostname;
    let name = gethostname().into_string().unwrap();
    let node = discovery::Node::new(name, &None)?;
    Ok(node)
}

impl cli::Command {
    pub async fn run(self) -> Result<()> {
        let cwd = std::env::current_dir()?;
        let client_node = setup_client_node()?;
        match self {
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
                let active_node = client_node.start_discovery();
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
            cli::Command::Start {
                addr,
                port,
                config_path,
            } => todo!(),
            cli::Command::Print {
                machine_id,
                device_name,
                gcode_path,
            } => todo!(),
            cli::Command::Log {
                machine_id,
                device_name,
            } => todo!(),
            cli::Command::Connect {
                machine_id,
                device_name,
            } => todo!(),
        };
        Ok(())
    }
}
