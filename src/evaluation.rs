use crate::context::Context;
use crate::states::*;
use crate::utils::Signal;

struct EvaluationStateMachine {
    current_state: Option<Box<dyn State>>
}

impl EvaluationStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: Some(Box::new(InitState))
        }
    }

    fn step(&mut self, word: &str) -> Result<(), Signal> {
        if let Some(s) = self.current_state.take() {
            let next_state = s.next(word)?;
            self.current_state = Some(next_state);
        }
        Ok(())
    }

    fn finish(&mut self, ctx: &mut Context) {
        if let Some(state) = &mut self.current_state {
            state.end(ctx)
        }
    }
}

pub fn evaluate(line: &str, ctx: &mut Context) {
    let mut sm = EvaluationStateMachine::new();
    for word in line.split_whitespace() {
        if let Err(e) = sm.step(word) {
            eprintln!("{}", e);
            return;
        }
    }
    sm.finish(ctx)
}