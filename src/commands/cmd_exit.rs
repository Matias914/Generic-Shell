use super::Command;
use crate::context::Context;

pub struct ExitCommand {
    exit_code: i32,
    has_argument: bool,
    too_many_arguments: bool
}

impl ExitCommand {
    pub fn new() -> Self {
        Self {
            exit_code: 0,
            has_argument: false,
            too_many_arguments: false,
        }
    }
}

impl Command for ExitCommand {
    fn execute(&mut self, ctx: &mut Context) {
        if self.too_many_arguments {
            println!("exit: too many arguments!");
            return;
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
}