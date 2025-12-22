use super::Command;
use crate::context::Context;

pub struct EchoCommand {
    arguments: String,
}

impl EchoCommand {
    pub fn new() -> Self {
        Self { arguments: String::new() }
    }
}

impl Command for EchoCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if ! self.arguments.is_empty() {
            println!("{}", self.arguments);
        }
    }

    fn add_argument(&mut self, arg: &str) {
        if ! self.arguments.is_empty() {
            self.arguments.push(' ');
        }
        self.arguments.push_str(arg);
    }
}