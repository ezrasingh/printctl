mod error;
mod prelude;

mod agent;
mod cli;
mod printer;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;
    use tokio::fs;

    use agent::PrintAgent;
    use cli::{Cli, Command};

    let cli = Cli::parse();
    let agent_name = hostname::get()?.into_string().unwrap_or("localhost".into());
    let mut local_agent = PrintAgent::new(&agent_name);

    match cli.command {
        Command::Ui {
            use_web,
            addr,
            port,
        } => {
            if use_web {
                let addr = addr.unwrap_or_else(|| "0.0.0.0".parse().unwrap());
                let port = port.unwrap_or(8020);
                printctl_ui::web::start(addr, port).expect("could not start web frontend");
            } else {
                printctl_ui::tui::start().expect("could not start terminal frontend");
            }
        }

        Command::ListDevices => {
            let devices = local_agent
                .available_devices()
                .expect("Could not list devices");
            println!("{:#?}", devices);
        }

        Command::UploadGcode { file } => {
            let bytes = fs::read(&file).await?;
            let file_name = file.file_name().expect("Could not determine filename");

            let id = local_agent.upload_gcode(file_name, bytes);
            println!("Uploaded GCODE as ID {}", id);
        }

        Command::QueueJob {
            printer_id,
            gcode_id,
        } => {
            let id = local_agent.create_job(printer_id, gcode_id);
            println!("Queued job {}", id);
        }

        Command::ListJobs => {
            for job in local_agent.list_jobs() {
                println!("{:?}", job);
            }
        }
    }

    Ok(())
}
