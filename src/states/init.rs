use super::{State, Signal};
use crate::commands::factory::Factory;
use crate::context::Context;
use crate::states::running::RunningState;

pub struct InitState;

impl State for InitState {
    fn next(self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal> {
        let command = Factory::get_command(word)?;
        Ok(Box::new(RunningState::new(command)))
    }

    fn end(&mut self, ctx: &mut Context) {
        ctx.set_running_state(true)
    }
}