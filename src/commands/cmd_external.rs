use super::{Command};
use crate::context::Context;

pub struct ExternalCommand {
    command: std::process::Command,
}

impl ExternalCommand {
    pub fn new(name: &str) -> Self {
        Self {
            command: std::process::Command::new(name),
        }
    }
}

impl Command for ExternalCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        self.command.stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .stdin(std::process::Stdio::inherit())
            .status()
            .expect("something went wrong when executing external command");
    }

    fn add_argument(&mut self, arg: &str) {
        self.command.arg(arg);
    }
}