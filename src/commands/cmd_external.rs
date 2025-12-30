use std::process::Stdio;
use super::{Command, Output};
use crate::utils::context::Context;

pub struct ExternalCommand {
    command: std::process::Command,
}

impl ExternalCommand {
    pub fn new(name: &str) -> Self {
        let mut command = std::process::Command::new(name);
        command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit());
        Self {
            command,
        }
    }
}

impl Command for ExternalCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        self.command.status().expect("something went wrong");
    }

    fn add_argument(&mut self, arg: &str) {
        self.command.arg(arg);
    }

    fn stdin(&mut self, input: Stdio) {
        let stdin = Stdio::from(input);
        self.command.stdin(stdin);
    }

    fn stdout(&mut self, output: Output) {
        self.command.stdout(output.stdio());
    }

    fn stderr(&mut self, error: Output) {
        self.command.stderr(error.stdio());
    }
}