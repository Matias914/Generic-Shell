use std::process::Stdio;
use crate::commands::{Command, Output};
use crate::utils::writer::Writer;
use crate::utils::context::Context;

pub struct CdCommand {
    path: String,
    has_argument: bool,
    too_many_arguments: bool,
    writer: Writer,
}

impl CdCommand {
    pub fn new() -> CdCommand {
        Self {
            path: String::new(),
            has_argument: false,
            too_many_arguments: false,
            writer: Writer::new(),
        }
    }
}

impl Command for CdCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if self.too_many_arguments {
            self.writer.log("cd: Too many arguments!");
            return
        }
        if self.path.eq("~") {
            let Ok(home) = std::env::var("HOME") else {
                self.writer.log("cd: Unable to read home directory");
                return
            };
            self.path = home;
        }
        if let Err(_) = std::env::set_current_dir(&self.path) {
            let message = &format!("cd: {}: No such file or directory", self.path);
            self.writer.log(message);
            return
        }
    }

    fn add_argument(&mut self, arg: &str) {
        if self.has_argument {
            self.too_many_arguments = true;
            return
        }
        self.path = arg.into();
        self.has_argument = true
    }

    fn stdin(&mut self, _input: Stdio) {
        // 'cd' doesn't input files
    }

    fn stdout(&mut self, _output: Output) {
        // 'cd' doesn't output files
    }

    fn stderr(&mut self, error: Output) {
        self.writer.set_log(error);
    }
}