mod commands;
mod evaluation;
mod repl;
mod states;
mod utils;
mod context;

fn main() {
    let exit_code = repl::start();
    std::process::exit(exit_code);
}
