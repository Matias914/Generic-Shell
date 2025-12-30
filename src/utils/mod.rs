pub mod searcher;
pub mod context;
pub mod writer;

use std::fmt;

#[derive(Debug)]
pub enum Signal {
    FileNotFound    { file:    String },
    CommandNotFound { command: String },
    SyntaxError     { message: String },
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signal::FileNotFound { file } => {
                write!(f, "msh: no such file or directory: {}", file)
            }
            Signal::CommandNotFound { command } => {
                write!(f, "{}: command not found", command)
            }
            Signal::SyntaxError { message } => {
                write!(f, "msh: syntax error: {}", message)
            }
        }
    }
}