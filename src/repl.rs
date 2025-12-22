use std::io::Write;
use crate::context::Context;
use crate::evaluation::evaluate;

pub fn start() -> i32 {
    let mut ctx = Context::new();
    let mut input = String::new();

    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        input.clear();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Valid input");

        let str_input = input.trim();

        if str_input.len() != 0 {
            evaluate(str_input, &mut ctx);
        }
        
        if ctx.should_stop() {
            return ctx.get_exit_code();
        }
    }
}