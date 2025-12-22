pub mod searcher;

use std::fmt;

#[derive(Debug)]
pub enum Signal {
    CommandNotFound { command: String },
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signal::CommandNotFound { command } => {
                write!(f, "{}: command not found", command)
            }
        }
    }
}