use crate::input::Action;
use crate::utils::context::Context;

pub struct BackspaceAction;
pub struct CharAction {
    character: char
}
pub struct CtrlCAction;
pub struct CtrlJAction;
pub struct DeleteAction;
pub struct DownAction;
pub struct EnterAction;
pub struct LeftAction;
pub struct RightAction;
pub struct UpAction;

impl BackspaceAction {
    pub fn new() -> Self { Self }
}

impl CharAction {
    pub fn new() -> Self { Self { character: '\0' } }

    pub fn set(&mut self, character: char) {
        self.character = character;
    }
}

impl CtrlCAction {
    pub fn new() -> Self { Self }
}

impl CtrlJAction {
    pub fn new() -> Self { Self }
}

impl DeleteAction {
    pub fn new() -> Self { Self }
}
impl DownAction {
    pub fn new() -> Self { Self }
}

impl EnterAction {
    pub fn new() -> Self { Self }
}

impl LeftAction {
    pub fn new() -> Self { Self }
}

impl RightAction {
    pub fn new() -> Self { Self }
}

impl UpAction {
    pub fn new() -> Self { Self }
}

impl Action for BackspaceAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, _ctx: &mut Context) {
        if *cursor_position > 0 {
            *cursor_position -= 1;
            input.remove(*cursor_position);
            let tail = &input[*cursor_position..];
            BackspaceAction::buffer(&format!("\x08{} \x08", tail));
            for _ in 0..tail.len() {
                BackspaceAction::buffer("\x08");
            }
            BackspaceAction::flush();
        }
    }
}

impl Action for CharAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, _ctx: &mut Context) {
        input.insert(*cursor_position, self.character);
        let tail = &input[*cursor_position..];
        *cursor_position += 1;
        CharAction::buffer(&format!("{}", tail));
        for _ in 0..tail.len() - 1 {
            CharAction::buffer("\x08");
        }
        CharAction::flush();
    }
}

impl Action for CtrlCAction {
    fn execute(&mut self, input: &mut String, _cursor_position: &mut usize, _ctx: &mut Context) {
        input.clear();
        CtrlCAction::print("^C\r\n$ ");
    }
}

impl Action for CtrlJAction {
    fn execute(&mut self, _input: &mut String, _cursor_position: &mut usize, _ctx: &mut Context) {
        CtrlJAction::print("\r\n");
    }
}

impl Action for DeleteAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, _ctx: &mut Context) {
        if *cursor_position < input.len() {
            input.remove(*cursor_position);
            let tail = &input[*cursor_position..];
            DeleteAction::buffer(&format!("{} \x08", tail));
            for _ in 0..tail.len() {
                DeleteAction::buffer("\x08");
            }
            DeleteAction::flush();
        }
    }
}

impl Action for DownAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, ctx: &mut Context) {
        let mut cmd = String::new();

        let result = ctx.history().next();
        if let Some(value) = result {
            cmd = value.to_string();
        }
        else {
            if input.is_empty() {
                return
            }
        }

        // Moves cursor to start
        for _ in 0..*cursor_position {
            DownAction::print("\x08");
        }

        let before_size = input.len();

        input.clear();
        input.push_str(&cmd);
        *cursor_position = input.len();

        // Overrides old input
        DownAction::print(input);

        // Removes residuals if any
        let residuals: i32 = before_size as i32 - input.len() as i32;
        for _ in 0..residuals {
            DownAction::print("\x1b[1C");
        }
        for _ in 0..residuals {
            DownAction::print("\x08 \x08");
        }
    }
}

impl Action for EnterAction {
    fn execute(&mut self, _input: &mut String, _cursor_position: &mut usize, _ctx: &mut Context) {
        EnterAction::print("\r\n");
    }
}

impl Action for LeftAction {
    fn execute(&mut self, _input: &mut String, cursor_position: &mut usize, _ctx: &mut Context) {
        if *cursor_position > 0 {
            *cursor_position -= 1;
            LeftAction::print("\x08");
        }
    }
}

impl Action for RightAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, _ctx: &mut Context) {
        if *cursor_position < input.len() {
            *cursor_position += 1;
            RightAction::print("\x1b[1C");
        }
    }
}

impl Action for UpAction {
    fn execute(&mut self, input: &mut String, cursor_position: &mut usize, ctx: &mut Context) {
        let Some(cmd) = ctx.history().back().map(|s| s.to_string()) else {
            return;
        };

        // Moves cursor to start
        for _ in 0..*cursor_position {
            UpAction::print("\x08");
        }

        let before_size = input.len();

        input.clear();
        input.push_str(&cmd);
        *cursor_position = input.len();

        // Overrides old input
        UpAction::print(input);

        // Removes residuals if any
        let residuals: i32 = before_size as i32 - input.len() as i32;
        for _ in 0..residuals {
            UpAction::print("\x1b[1C");
        }
        for _ in 0..residuals {
            UpAction::print("\x08 \x08");
        }
    }
}

