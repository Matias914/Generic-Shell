use crate::commands::Command;
use crate::context::Context;

pub struct CdCommand {
    path: String,
    has_argument: bool,
    too_many_arguments: bool,
    
}

impl CdCommand {
    pub fn new() -> CdCommand {
        Self {
            path: String::new(),
            has_argument: false,
            too_many_arguments: false,
        }
    }
}

impl Command for CdCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if self.too_many_arguments {
            println!("cd: too many arguments!");
            return
        }
        if self.path.eq("~") {
            let Ok(home) = std::env::var("HOME") else {
                println!("cd: unable to read home directory");
                return
            };
            self.path = home;
        }
        if let Err(_) = std::env::set_current_dir(&self.path) {
            println!("cd: {}: No such file or directory", self.path);
            return
        }
    }

    fn add_argument(&mut self, arg: &str) {
        if self.has_argument {
            self.too_many_arguments = true;
            return
        }
        self.path = arg.into();
        self.has_argument = true
    }
}