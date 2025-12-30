use std::fs::{File, OpenOptions};
use std::process::Stdio;
use std::thread;
use std::thread::JoinHandle;
use os_pipe::PipeReader;
use super::{State};
use crate::commands::{Command, Noop, Output};
use crate::commands::factory::Factory;
use crate::utils::context::Context;
use crate::utils::{Signal};
use crate::utils::Signal::*;

pub(super) struct InitState;
pub(super) struct RunningState { command: Box<dyn Command> }
pub(super) struct PipeState { jobs: Vec<JoinHandle<i32>>, reader: PipeReader }
pub(super) struct PipeRunningState { jobs: Vec<JoinHandle<i32>>, command: Box<dyn Command> }
pub(super) struct RedirectionState { command: Box<dyn Command>, operator: String }

impl RunningState {
    pub fn new(command: Box<dyn Command>) -> Self {
        Self { command }
    }
}

impl PipeState {
    pub fn new(jobs: Vec<JoinHandle<i32>>, reader: PipeReader) -> Self {
        Self {
            jobs,
            reader
        }
    }
}

impl PipeRunningState {
    pub fn new(jobs: Vec<JoinHandle<i32>>, command: Box<dyn Command>) -> Self {
        Self {
            jobs,
            command,
        }
    }
}

impl RedirectionState {
    pub fn new(command: Box<dyn Command>, operator: String) -> Self {
        Self { command, operator }
    }
}

impl State for InitState {
    fn shift(self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        let command = Factory::get(word)?;
        Ok(Box::new(RunningState::new(command)))
    }

    fn reduce(&mut self, _ctx: &mut Context) -> Result<(), Signal> {
        // Nothing reduces to anything
        Ok(())
    }
}

impl State for RunningState {
    fn shift(mut self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        let last_byte = word.as_bytes().last().unwrap();
        match *last_byte {
            b'|' => {
                let (reader, writer) = os_pipe::pipe().unwrap();
                self.command.stdout(Output::Pipe(writer));

                let mut cmd = std::mem::replace(&mut self.command, Box::new(Noop));
                let job = thread::spawn(move || {
                    cmd.execute(&mut Context::new());
                    0
                });

                let mut jobs = Vec::new();
                jobs.insert(0, job);
                Ok(Box::new(PipeState::new(jobs, reader)))
            }
            b'<' | b'>' => {
                Ok(Box::new(RedirectionState::new(self.command, word.into())))
            }
            _ => {
                self.command.add_argument(word);
                Ok(self)
            }
        }
    }

    fn reduce(&mut self, ctx: &mut Context) -> Result<(), Signal> {
        self.command.execute(ctx);
        Ok(())
    }
}

impl State for PipeState {
    fn shift(self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        match Factory::get(word) {
            Err(e) => {
                println!("{}", e);
                Ok(Box::new(PipeRunningState::new(self.jobs, Box::new(Noop))))
            },
            Ok(mut command) => {
                command.stdin(Stdio::from(self.reader));
                Ok(Box::new(PipeRunningState::new(self.jobs, command)))
            }
        }
    }

    fn reduce(&mut self, _ctx: &mut Context) -> Result<(), Signal> {
        Err(SyntaxError { message: "expected command after |".to_string() })
    }
}

impl State for PipeRunningState {
    fn shift(mut self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        let last_byte = word.as_bytes().last().unwrap();
        match *last_byte {
            b'|' => {
                let (reader, writer) = os_pipe::pipe().unwrap();
                self.command.stdout(Output::Pipe(writer));

                let mut cmd = std::mem::replace(&mut self.command, Box::new(Noop));
                let job = thread::spawn(move || {
                    cmd.execute(&mut Context::new());
                    0
                });

                self.jobs.push(job);
                Ok(Box::new(PipeState::new(self.jobs, reader)))
            }
            b'<' | b'>' => {
                Ok(Box::new(RedirectionState::new(self.command, word.into())))
            }
            _ => {
                self.command.add_argument(word);
                Ok(self)
            }
        }
    }

    fn reduce(&mut self, ctx: &mut Context) -> Result<(), Signal> {
        self.command.execute(ctx);

        // Kills reader without leaving self.command empty
        let _ = std::mem::replace(&mut self.command, Box::new(Noop));

        for job in self.jobs.drain(..) {
            job.join().unwrap();
        }

        Ok(())
    }
}

impl State for RedirectionState {
    fn shift(mut self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        match self.operator.as_str() {
            "<" => {
                let Ok(file) = File::open(word) else {
                    return Err(FileNotFound { file: word.into() });
                };
                self.command.stdin(Stdio::from(file));
            }
            "2>" => {
                let Ok(file) = File::create(word) else {
                    return Err(FileNotFound { file: word.into() });
                };
                self.command.stderr(Output::File(file));
            }
            "2>>" => {
                let result = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(word);
                let Ok(file) = result else {
                    return Err(FileNotFound { file: word.into() });
                };
                self.command.stderr(Output::File(file));
            }
            ">" | "1>" => {
                let Ok(file) = File::create(word) else {
                    return Err(FileNotFound { file: word.into() });
                };
                self.command.stdout(Output::File(file));
            }
            ">>" | "1>>" => {
                let result = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(word);
                let Ok(file) = result else {
                    return Err(FileNotFound { file: word.into() });
                };
                self.command.stdout(Output::File(file));
            }
            _ => {
                return Err(SyntaxError { message: "unknown redirection operator".into() })
            }
        }
        Ok(Box::new(RunningState::new(self.command)))
    }

    fn reduce(&mut self, _ctx: &mut Context) -> Result<(), Signal> {
        Err(SyntaxError { message: "expected redirection".into() })
    }
}
  