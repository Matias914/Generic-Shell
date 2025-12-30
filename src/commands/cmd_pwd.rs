use std::process::Stdio;
use crate::commands::{Command, Output};
use crate::utils::context::Context;
use crate::utils::writer::Writer;

pub struct PwdCommand {
    too_many_arguments: bool,
    writer: Writer,
}

impl PwdCommand {
    pub fn new() -> PwdCommand {
        Self {
            too_many_arguments: false,
            writer: Writer::new(),
        }
    }
}

impl Command for PwdCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if self.too_many_arguments {
            self.writer.log("pwd: too many arguments!");
            return
        }
        let Ok(path) = std::env::current_dir() else {
            self.writer.log("pwd: unable to read current directory");
            return
        };
        let Some(str_path) = path.to_str() else {
            self.writer.log("pwd: failed to convert current path to string format");
            return
        };
        self.writer.show(&str_path.into());
    }

    fn add_argument(&mut self, _arg: &str) {
        self.too_many_arguments = true;
    }

    fn stdin(&mut self, _input: Stdio) {
        // 'pwd' doesn't input files
    }

    fn stdout(&mut self, output: Output) {
        self.writer.set_output(output)
    }

    fn stderr(&mut self, error: Output) {
        self.writer.set_log(error)
    }
}