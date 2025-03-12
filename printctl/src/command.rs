use crate::prelude::*;

use crate::cli::Command;

pub struct CommandRunner(Command);

impl From<Command> for CommandRunner {
    fn from(cmd: Command) -> Self {
        Self(cmd)
    }
}

impl CommandRunner {
    pub fn run(self) -> Result<()> {
        let cwd = std::env::current_dir()?;
        match self.0 {
            Command::Print {
                gcode,
                http_endpoint,
            } => {
                todo!()
            }
        };
        Ok(())
    }
}
