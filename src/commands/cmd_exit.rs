use std::process::Stdio;
use super::{Command, Output};
use crate::utils::context::Context;
use crate::utils::writer::Writer;

pub struct ExitCommand {
    exit_code: i32,
    has_argument: bool,
    too_many_arguments: bool,
    writer: Writer
}

impl ExitCommand {
    pub fn new() -> Self {
        Self {
            exit_code: 0,
            has_argument: false,
            too_many_arguments: false,
            writer: Writer::new()
        }
    }
}

impl Command for ExitCommand {
    fn execute(&mut self, ctx: &mut Context) {
        if self.too_many_arguments {
            self.writer.log("exit: too many arguments!");
            return
        }
        ctx.set_running_state(false);
        ctx.set_exit_code(self.exit_code);
    }

    fn add_argument(&mut self, arg: &str) {
        if self.has_argument {
            self.too_many_arguments = true;
            return
        }
        let Ok(exit_code) = arg.parse::<i32>() else {
            self.has_argument = true;
            self.too_many_arguments = true;
            return
        };
        self.exit_code = exit_code;
        self.has_argument = true
    }

    fn stdin(&mut self, _input: Stdio) {
        // 'exit' doesn't input anything
    }

    fn stdout(&mut self, _output: Output) {
        // 'exit' doesn't output anything
    }

    fn stderr(&mut self, error: Output) {
        self.writer.set_log(error);
    }
}