use crate::prelude::*;

use crate::cli;
use crate::dashboard;

pub struct CommandRunner(cli::Command);

impl From<cli::Command> for CommandRunner {
    fn from(cmd: cli::Command) -> Self {
        Self(cmd)
    }
}

impl CommandRunner {
    pub async fn run(self) -> Result<()> {
        use gethostname::gethostname;
        use printctl_node::mesh::Idle;

        let cwd = std::env::current_dir()?;
        let cluster_node =
            printctl_node::mesh::Node::<Idle>::new(gethostname().into_string().unwrap(), &None)?;
        match self.0 {
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
                let active_node = cluster_node.start_discovery();
                for _ in 0..20 {
                    println!("Listening for peers...");
                    let peers = active_node.peers().await;
                    if machines {
                        for peer in peers {
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
        };
        Ok(())
    }
}
