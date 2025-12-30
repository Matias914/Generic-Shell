use std::fs::{File, OpenOptions};
use std::mem::take;
use std::process::Stdio;
use crate::commands::{Command, Output};
use crate::utils::context::Context;
use crate::utils::writer::Writer;

pub struct HistoryCommand {
    writer: Writer,
    output: String,
    number: i32,
    file: Option<File>,
    has_argument: bool,
    option: char,
    too_many_arguments: bool
}

impl HistoryCommand {
    pub fn new() -> Self {
        Self {
            writer: Writer::new(),
            output: String::new(),
            number: -1,
            file: None,
            has_argument: false,
            option: 'n',
            too_many_arguments: false
        }
    }
}

impl Command for HistoryCommand {
    fn execute(&mut self, ctx: &mut Context) {
        if self.too_many_arguments {
            self.writer.log("history: too many arguments!");
            return
        }

        if let Some(file) = take(&mut self.file) {
            if self.option == 'r' {
                ctx.history().read(file);
                return
            }
            if self.option == 'w' {
                ctx.history().write(file);
                return
            }
            ctx.history().append(file);
            return
        }

        let mut first = true;
        self.output = String::new();

        // Last N numbers
        if self.number >= 0 {
            let len = ctx.history().len();
            for (i, cmd) in ctx.history().rev().enumerate() {
                if self.number <= i as i32 {
                    break;
                }
                if ! first {
                    self.output.insert(0, '\n');
                }
                self.output.insert_str(0,&format!("    {} {cmd}", len - i));
                first = false;
            }
            self.writer.show(&self.output);
            return
        }

        // Complete history
        for (i, cmd) in ctx.history().iter().enumerate() {
            if ! first {
                self.output.push_str("\n");
            }
            self.output.push_str(&format!("    {} {cmd}", i + 1));
            first = false;
        }
        self.writer.show(&self.output);
    }

    fn add_argument(&mut self, arg: &str) {
        if self.has_argument {
            self.too_many_arguments = true;
            return
        }
        // If it's a number
        if let Ok(code) = arg.parse::<i32>() {
            self.has_argument = true;
            if code >= 0 {
                self.number = code;
                return
            }
            self.too_many_arguments = true;
            return
        }
        // If it's not an option
        if arg != "-r" && arg != "-w" && arg != "-a" {
            self.has_argument = true;
            if let Ok(file) = match self.option {
                'w' => {
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(arg)
                }
                'a' => {
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(arg)
                }
                _ => {
                    File::open(arg)
                }
            } {
                self.file = Some(file);
                return
            }
            self.too_many_arguments = true;
        }
        match arg {
            "-r" if self.option == 'n' => {
                self.option = 'r'
            }
            "-w" if self.option == 'n' => {
                self.option = 'w'
            }
            "-a" if self.option == 'n' => {
                self.option = 'a'
            }
            _ => {
                self.has_argument = true;
                self.too_many_arguments = true;
            }
        }
    }

    fn stdin(&mut self, _input: Stdio) {
        // History doesn't input anything
    }

    fn stdout(&mut self, output: Output) {
        self.writer.set_output(output);
    }

    fn stderr(&mut self, output: Output) {
        self.writer.set_log(output);
    }
}