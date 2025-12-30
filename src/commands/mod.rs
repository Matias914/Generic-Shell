mod cmd_echo;
mod cmd_exit;
mod cmd_type;
mod cmd_external;
mod cmd_pwd;
mod cmd_cd;
pub mod factory;
mod cmd_history;

use os_pipe::PipeWriter;
use std::fs::File;
use std::io::{Write};
use std::process::{Stdio};
use crate::utils::context::Context;

pub trait Command: Send {
    fn execute(&mut self, ctx: &mut Context);
    fn add_argument(&mut self, arg: &str);
    fn stdin(&mut self, input: Stdio);
    fn stdout(&mut self, output: Output);
    fn stderr(&mut self, error: Output);
}

pub struct Noop;
impl Command for Noop {
    fn execute(&mut self, _ctx: &mut Context) {}
    fn add_argument(&mut self, _arg: &str) {}
    fn stdin(&mut self, _input: Stdio) {}
    fn stdout(&mut self, _output: Output) {}
    fn stderr(&mut self, _error: Output) {}
}

pub enum Output {
    File(File),
    Pipe(PipeWriter),
}

impl Output {
    pub fn stdio(self) -> Stdio {
        match self {
            Output::File(file) => Stdio::from(file),
            Output::Pipe(pipe) => Stdio::from(pipe),
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::File(f) => f.write(buf),
            Output::Pipe(p) => p.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::File(f) => f.flush(),
            Output::Pipe(p) => p.flush(),
        }
    }
}