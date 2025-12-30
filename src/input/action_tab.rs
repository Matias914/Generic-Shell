use crate::commands::factory::Factory;
use crate::input::Action;
use crate::utils::context::Context;

pub struct TabAction {
    show_possibilities: bool,
}

impl TabAction {
    pub fn new() -> TabAction {
        Self {
            show_possibilities: false
        }
    }
    
    fn longest_prefix(options: &Vec<String>) -> Option<String> {
        let Some(mut longest_prefix) = options.first().cloned() else {
            return None
        };
        for cmd in options {
            while !cmd.starts_with(longest_prefix.as_str()) {
                longest_prefix.pop();
                if longest_prefix.is_empty() {
                    return None
                }
            }
        }
        Some(longest_prefix)
    }
}

impl Action for TabAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, _ctx: &mut Context) {
        let incomplete = &input[..*cursor_position];
        let options = Factory::complete(incomplete);
        match options.len() {
            0 => {
                // Rings a bell
                TabAction::print("\x07");
            }
            1 => {
                // Completes command
                let mut suffix = options.iter()
                    .next()
                    .unwrap()
                    .strip_prefix(incomplete)
                    .unwrap()
                    .to_string();
                suffix.push(' ');
                input.insert_str(*cursor_position, &suffix);
                let tail = &input[*cursor_position..];
                *cursor_position += suffix.len();
                TabAction::buffer(tail);
                for _ in 0..tail.len() - suffix.len() {
                    TabAction::buffer("\x08");
                }
                TabAction::print("");
            }
            _ => {
                let Some(longest_prefix) = TabAction::longest_prefix(&options) else {
                    return
                };
                // Shows partial completion
                if longest_prefix != incomplete {
                    let suffix = longest_prefix.strip_prefix(incomplete).unwrap();
                    TabAction::print(suffix);
                    input.insert_str(*cursor_position, &suffix);
                    *cursor_position += suffix.len();
                    return
                }
                // Rings a bell
                if ! self.show_possibilities {
                    TabAction::print("\x07");
                    self.show_possibilities = true;
                    return
                }
                // Shows all possibilities
                TabAction::buffer("\r\n");
                for entry in options {
                    TabAction::buffer(&format!("{}  ", entry));
                }
                TabAction::print(&format!("\r\n$ {}", incomplete));
                self.show_possibilities = false;
            }
        }
    }
}