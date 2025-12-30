mod states;

use crate::utils::context::Context;
use crate::utils::Signal;
use crate::parser::states::InitState;

trait State {
    fn shift(self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal>;
    fn reduce(&mut self, ctx: &mut Context) -> Result<(), Signal>;
}

pub struct Parser {
    state: Option<Box<dyn State>>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(InitState))
        }
    }

    pub fn add(&mut self, word: &str) -> Result<(), Signal> {
        if let Some(curr_state) = self.state.take() {
            let next_state = curr_state.shift(word)?;
            self.state = Some(next_state);
        }
        Ok(())
    }

    pub fn eol(&mut self, ctx: &mut Context) -> Result<(), Signal> {
        if let Some(curr_state) = &mut self.state {
            return curr_state.reduce(ctx)
        }
        Ok(())
    }
}