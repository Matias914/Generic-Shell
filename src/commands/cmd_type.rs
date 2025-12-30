use std::process::Stdio;
use super::{Command, factory::Factory, Output};
use crate::utils::context::Context;
use crate::utils::searcher::Searcher;
use crate::utils::writer::Writer;

pub struct TypeCommand {
    response: String,
    writer: Writer,
}

impl TypeCommand {
    pub fn new() -> Self {
        Self { response: String::new(), writer: Writer::new() }
    }
}

impl Command for TypeCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if ! self.response.is_empty() {
            self.writer.show(&self.response);
        }
    }

    fn add_argument(&mut self, arg: &str) {
        if ! self.response.is_empty() {
            self.response.push('\n');
        }
        self.response.push_str(arg);
        if Factory::is_builtin(arg) {
            self.response.push_str(" is a shell builtin");
            return;
            
        }
        if let Some(location) = Searcher::search_executable_in_path(arg) {
            self.response.push_str(" is ");
            self.response.push_str(location.as_str());
        } else {
            self.response.push_str(": not found");
        }
    }

    fn stdin(&mut self, _input: Stdio) {
        // 'type' doesn't input files
    }
    
    fn stdout(&mut self, output: Output) {
        self.writer.set_output(output);
    }
    
    fn stderr(&mut self, error: Output) {
        self.writer.set_log(error);
    }
}