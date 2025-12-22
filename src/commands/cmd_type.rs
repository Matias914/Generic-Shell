use super::{Command, factory::Factory};
use crate::context::Context;
use crate::utils::searcher::ExecutableSearcher;

pub struct TypeCommand {
    response: String,
}

impl TypeCommand {
    pub fn new() -> Self {
        Self { response: String::new() }
    }
}

impl Command for TypeCommand {
    fn execute(&mut self, _ctx: &mut Context) {
        if ! self.response.is_empty() {
            println!("{}", self.response);
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
        if let Some(location) = ExecutableSearcher::search_in_path(arg) {
            self.response.push_str(" is ");
            self.response.push_str(location.as_str());
        } else {
            self.response.push_str(": not found");
        }
    }
}