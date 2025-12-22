use super::{State, Signal};
use crate::commands::Command;
use crate::context::Context;

pub struct RunningState {
    current_command: Box<dyn Command>,
}

impl RunningState {
    pub fn new(command: Box<dyn Command>) -> Self {
        Self { current_command: command }
    }
}

impl State for RunningState {
    fn next(mut self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        self.current_command.add_argument(word);
        Ok(self)
    }

    fn end(&mut self, ctx: &mut Context) {
        self.current_command.execute(ctx);
    }
}