use std::io::Write;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::input::LineReader;
use crate::utils::context::Context;

pub fn start() -> i32 {
    let mut ctx = Context::new();
    let mut input = String::new();
    let mut reader = LineReader::new();

    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        input.clear();
        reader.get(&mut input, &mut ctx);

        if input.len() != 0 {
            ctx.history().add(input.clone());
            evaluate(&input, &mut ctx);
        }
        
        if ctx.should_stop() {
            return ctx.get_exit_code();
        }
    }
}

pub fn evaluate(line: &str, ctx: &mut Context) {
    let mut lexer = Lexer::new(line);
    let mut sm = Parser::new();
    while let Some(word) = lexer.next() {
        if let Err(e) = sm.add(word) {
            eprintln!("{}", e);
            return;
        }
    }
    if let Err(e) = sm.eol(ctx) {
        eprintln!("{}", e);
        return;
    }
}