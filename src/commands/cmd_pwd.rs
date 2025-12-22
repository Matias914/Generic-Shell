use crate::commands::Command;
use crate::context::Context;

pub struct PwdCommand {
    too_many_arguments: bool
}

impl PwdCommand {
    pub fn new() -> PwdCommand {
        Self {
            too_many_arguments: false
        }
    }
}

impl Command for PwdCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if self.too_many_arguments {
            println!("pwd: too many arguments!");
            return
        }
        let Ok(path) = std::env::current_dir() else {
            println!("pwd: unable to read current directory");
            return
        };
        let Some(str_path) = path.to_str() else {
            println!("pwd: failed to convert current path to string format");
            return
        };
        println!("{}", str_path);
    }
    fn add_argument(&mut self, _arg: &str) {
        self.too_many_arguments = true;
    }
}