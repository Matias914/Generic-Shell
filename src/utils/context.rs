use crate::utils::history::History;

pub struct Context {
    running: bool,
    exit_code: i32,
    history: History
}

impl Context {
    pub fn new() -> Context {
        Self {
            running: true,
            exit_code: 0,
            history: History::new()
        }
    }

    pub fn set_running_state(&mut self, state: bool) {
        self.running = state;
    }

    pub fn set_exit_code(&mut self, code: i32) {
        self.exit_code = code;
    }
    
    pub fn should_stop(&self) -> bool {
        !self.running
    }

    pub fn get_exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn history(&mut self) -> &mut History {
        &mut self.history
    }
}