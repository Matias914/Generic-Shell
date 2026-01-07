mod commands;
mod repl;
mod parser;
mod utils;
mod lexer;
mod input;

fn main() {
    let exit_code = repl::start();
    std::process::exit(exit_code);
}
