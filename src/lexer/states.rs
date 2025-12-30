use super::{Lexer, State};

pub(super) struct FinalState;
pub(super) struct InitState;
pub(super) struct FileDescriptorState;
pub(super) struct WaitingFileDescriptorState;
pub(super) struct WordState;
pub(super) struct WordsBackslashState;
pub(super) struct SingleQuoteState;
pub(super) struct DoubleQuoteState;
pub(super) struct DoubleQuoteBackslashState;
pub(super) struct WaitingSingleQuoteState;
pub(super) struct WaitingDoubleQuoteState;

impl State for FinalState {
    fn next(self: Box<Self>, _lexer: &mut Lexer) -> Box<dyn State> {
        self
    }
    fn is_final_state(&self) -> bool { true }
}
impl State for InitState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            ' ' | '\n' | '\t' => {
                lexer.skip();
                self
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                lexer.increment(char);
                Box::new(FileDescriptorState)
            }
            '>' => {
                lexer.increment(char);
                Box::new(WaitingFileDescriptorState)
            }
            '|' | '<' => {
                lexer.increment(char);
                Box::new(FinalState)
            }
            '\\' => {
                lexer.skip();
                Box::new(WordsBackslashState)
            }
            '\'' => {
                lexer.skip();
                Box::new(SingleQuoteState)
            }
            '"' => {
                lexer.skip();
                Box::new(DoubleQuoteState)
            }
            _ => {
                lexer.increment(char);
                Box::new(WordState)
            }
        }
    }
    fn is_final_state(&self) -> bool { false }
}

impl State for FileDescriptorState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            ' ' | '\n' | '\t' | '|' => {
                Box::new(FinalState)
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                lexer.increment(char);
                self
            }
            '<' => {
                lexer.increment(char);
                Box::new(FinalState)
            }
            '>' => {
                lexer.increment(char);
                Box::new(WaitingFileDescriptorState)
            }
            _ => {
                lexer.increment(char);
                Box::new(WordState)
            }
        }
    }
    fn is_final_state(&self) -> bool { false }
}

impl State for WaitingFileDescriptorState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            '>' => {
                lexer.increment(char);
                Box::new(FinalState)
            }
            _ => {
                Box::new(FinalState)
            }
        }
    }
    fn is_final_state(&self) -> bool { false }
}

impl State for WordState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            ' ' | '\n' | '\t' | '<' | '|' | '>' => {
                Box::new(FinalState)
            }
            '\\' => {
                lexer.skip();
                Box::new(WordsBackslashState)
            }
            '\'' => {
                lexer.skip();
                Box::new(SingleQuoteState)
            }
            '"' => {
                lexer.skip();
                Box::new(DoubleQuoteState)
            }
            _ => {
                lexer.increment(char);
                self
            }
        }
    }
    fn is_final_state(&self) -> bool { false }
}

impl State for WordsBackslashState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            _ => {
                lexer.increment(char);
                Box::new(WordState)
            }
        }
    }

    fn is_final_state(&self) -> bool {
        false
    }
}

impl State for SingleQuoteState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            '\'' => {
                lexer.skip();
                Box::new(WaitingSingleQuoteState)
            }
            _ => {
                lexer.increment(char);
                self
            }
        }
    }

    fn is_final_state(&self) -> bool { false }
}

impl State for DoubleQuoteState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            '"' => {
                lexer.skip();
                Box::new(WaitingDoubleQuoteState)
            }
            '\\' => {
                lexer.increment(char);
                Box::new(DoubleQuoteBackslashState)
            }
            _ => {
                lexer.increment(char);
                self
            }
        }
    }

    fn is_final_state(&self) -> bool { false }
}

impl State for DoubleQuoteBackslashState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            '"' | '\\' => {
                lexer.remove();
                lexer.increment(char);
                Box::new(DoubleQuoteState)
            }
            _ => {
                lexer.increment(char);
                Box::new(DoubleQuoteState)
            }
        }
    }

    fn is_final_state(&self) -> bool {
        false
    }
}

impl State for WaitingSingleQuoteState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            ' ' | '\n' | '\t' | '<' | '|' | '>' => {
                Box::new(FinalState)
            }
            '\\' => {
                lexer.skip();
                Box::new(WordsBackslashState)
            }
            '\'' => {
                lexer.skip();
                Box::new(SingleQuoteState)
            }
            _ => {
                Box::new(WordState)
            }
        }
    }

    fn is_final_state(&self) -> bool { false }
}

impl State for WaitingDoubleQuoteState {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State> {
        let Some(char) = lexer.get() else {
            return Box::new(FinalState);
        };
        match char {
            ' ' | '\n' | '\t' | '<' | '|' | '>' => {
                Box::new(FinalState)
            }
            '\\' => {
                lexer.skip();
                Box::new(WordsBackslashState)
            }
            '"' => {
                lexer.skip();
                Box::new(DoubleQuoteState)
            }
            _ => {
                Box::new(WordState)
            }
        }
    }

    fn is_final_state(&self) -> bool { false }
}