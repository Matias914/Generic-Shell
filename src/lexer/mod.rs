mod states;

use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::states::InitState;

trait State {
    fn next(self: Box<Self>, lexer: &mut Lexer) -> Box<dyn State>;
    fn is_final_state(&self) -> bool;
}

pub struct Lexer<'a> {
    lexeme: String,
    iterator: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            lexeme: String::new(),
            iterator: line.chars().peekable(),
        }
    }

    pub fn next(&mut self) -> Option<&str> {
        if ! self.lexeme.is_empty() {
            self.lexeme = String::new();
        }
        let mut state: Box<dyn State> = Box::new(InitState);
        while ! state.is_final_state() {
            state = state.next(self);
        }
        if self.lexeme.is_empty() && self.iterator.peek().is_none() {
            return None;
        }
        Some(&self.lexeme)
    }

    pub(super) fn skip(&mut self) {
        self.iterator.next();
    }

    pub(super) fn increment(&mut self, char: char) {
        self.iterator.next();
        self.lexeme.push(char);
    }
    
    pub(super) fn remove(&mut self) {
        self.lexeme.pop();
    }

    pub(super) fn get(&mut self) -> Option<char> {
        self.iterator.peek().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn test_next() {
        let mut lexer = Lexer::new (
            " \
            echo    hello world  cat ñ  289 'hola  a'> 123< 123a"
        );

        let word = lexer.next();
        assert_eq!(word, Some("echo"), "Must return 'echo'");

        let word = lexer.next();
        assert_eq!(word, Some("hello"), "Must return 'hello'");

        let word = lexer.next();
        assert_eq!(word, Some("world"), "Must return 'world'");

        let word = lexer.next();
        assert_eq!(word, Some("cat"), "Must return 'cat'");

        let word = lexer.next();
        assert_eq!(word, Some("ñ"), "Must return 'ñ'");

        let word = lexer.next();
        assert_eq!(word, Some("289"), "Must return '289'");

        let word = lexer.next();
        assert_eq!(word, Some("hola  a"), "Must return 'hola  a'");

        let word = lexer.next();
        assert_eq!(word, Some(">"), "Must return '>'");

        let word = lexer.next();
        assert_eq!(word, Some("123<"), "Must return '123<'");

        let word = lexer.next();
        assert_eq!(word, Some("123a"), "Must return '123a'");

        let word = lexer.next();
        assert_eq!(word, None, "Must return None (EOF)");

        let word = lexer.next();
        assert_eq!(word, None, "Must return None (EOF) again");
    }
}