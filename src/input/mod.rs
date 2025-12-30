mod action_tab;
mod actions;

use std::io::Write;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::input::action_tab::TabAction;
use crate::input::actions::*;
use crate::utils::context::Context;

trait Action {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, ctx: &mut Context);

    fn print(str: &str) {
        print!("{}", str);
        std::io::stdout().flush().unwrap();
    }

    fn buffer(str: &str) {
        print!("{}", str);
    }
    
    fn flush() {
        std::io::stdout().flush().unwrap();
    }
}

pub struct LineReader {
    backspace: BackspaceAction,
    chars: CharAction,
    ctrl_c: CtrlCAction,
    ctrl_j: CtrlJAction,
    delete: DeleteAction,
    down: DownAction,
    enter: EnterAction,
    left: LeftAction,
    right: RightAction,
    tab: TabAction,
    up: UpAction,
}

impl LineReader {
    pub fn new() -> LineReader{
        Self {
            backspace: BackspaceAction::new(),
            chars: CharAction::new(),
            ctrl_c: CtrlCAction::new(),
            ctrl_j: CtrlJAction::new(),
            delete: DeleteAction::new(),
            down: DownAction::new(),
            enter: EnterAction::new(),
            left: LeftAction::new(),
            right: RightAction::new(),
            tab: TabAction::new(),
            up: UpAction::new(),
        }
    }
}

impl LineReader {
    pub fn get(&mut self, input: &mut String, ctx: &mut Context) {
        // Unsafe mode for input detection in real time
        enable_raw_mode().unwrap();

        let mut cursor_position = 0;

        loop {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    // Ctrl+C
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.ctrl_c.execute(input, &mut cursor_position, ctx);
                    }
                    // Ctrl+J or Newline
                    KeyCode::Char('j') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.ctrl_j.execute(input, &mut cursor_position, ctx);
                        break;
                    }
                    // Autocompletion logic
                    KeyCode::Tab => {
                        self.tab.execute(input, &mut cursor_position, ctx);
                    }
                    // Shows backspace and removes chars from input
                    KeyCode::Backspace => {
                        self.backspace.execute(input, &mut cursor_position, ctx);
                    }
                    // Shows delete and removes chars from input
                    KeyCode::Delete => {
                        self.delete.execute(input, &mut cursor_position, ctx);
                    }
                    // Shows new line in screen and breaks loop
                    KeyCode::Enter => {
                        self.enter.execute(input, &mut cursor_position, ctx);
                        break;
                    }
                    // Shows the char and pushes it to input
                    KeyCode::Char(c) => {
                        self.chars.set(c);
                        self.chars.execute(input, &mut cursor_position, ctx);()
                    }
                    // Moves cursor to left
                    KeyCode::Left => {
                        self.left.execute(input, &mut cursor_position, ctx);
                    }
                    // Moves cursor to right
                    KeyCode::Right => {
                        self.right.execute(input, &mut cursor_position, ctx);
                    }
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::Up | KeyCode::PageUp => {
                        self.up.execute(input, &mut cursor_position, ctx);
                    }
                    KeyCode::Down | KeyCode::PageDown => {
                        self.down.execute(input, &mut cursor_position, ctx);
                    }
                    KeyCode::BackTab => {}
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Null => {}
                    KeyCode::Esc => {}
                    KeyCode::CapsLock => {}
                    KeyCode::ScrollLock => {}
                    KeyCode::NumLock => {}
                    KeyCode::PrintScreen => {}
                    KeyCode::Pause => {}
                    KeyCode::Menu => {}
                    KeyCode::KeypadBegin => {}
                    KeyCode::Media(_) => {}
                    KeyCode::Modifier(_) => {},
                }
            }
        }
        // End of unsafe mode
        disable_raw_mode().unwrap();
    }
}