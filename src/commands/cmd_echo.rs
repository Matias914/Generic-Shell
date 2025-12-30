use std::process::Stdio;
use super::{Command, Output};
use crate::utils::context::Context;
use crate::utils::writer::Writer;

pub struct EchoCommand {
    arguments: String,
    writer: Writer,
}

impl EchoCommand {
    pub fn new() -> Self {
        Self {
            arguments: String::new(),
            writer: Writer::new(),
        }
    }
}

impl Command for EchoCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        self.writer.show(&self.arguments);
    }

    fn add_argument(&mut self, arg: &str) {
        if ! self.arguments.is_empty() {
            self.arguments.push(' ');
        }
        self.arguments.push_str(arg);
    }

    fn stdin(&mut self, _input: Stdio) {
        // 'echo' doesn't input files
    }
    fn stdout(&mut self, output: Output) {
        self.writer.set_output(output);
    }

    fn stderr(&mut self, _error: Output) {
        // 'echo' doesn't output errors
    }

}