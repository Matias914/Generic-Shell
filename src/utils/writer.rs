use std::io::Write;
use crate::commands::Output;

pub struct Writer {
    output: Option<Output>,
    log: Option<Output>,
}

impl Writer {
    pub fn new() -> Writer {
        Self { output: None, log: None }
    }

    pub fn set_output(&mut self, output: Output) {
        if self.output.is_none() {
            self.output = Some(output);
        }
    }

    pub fn set_log(&mut self, log: Output) {
        if self.log.is_none() {
            self.log = Some(log);
        }
    }

    pub fn show(&mut self, message: &String) {
        if let Some(output) = self.output.as_mut() {
            writeln!(output, "{}", message).unwrap_or_default();
        } else {
            println!("{}", message);
        }
    }

    pub fn log(&mut self, message: &str) {
        if let Some(log) = self.log.as_mut() {
            writeln!(log, "{}", message).unwrap_or_default();
        } else {
            println!("{}", message);
        }
    }
}