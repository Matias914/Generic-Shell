use crate::commands::cmd_cd::CdCommand;
use crate::commands::cmd_echo::EchoCommand;
use crate::commands::cmd_exit::ExitCommand;
use crate::commands::cmd_external::ExternalCommand;
use crate::commands::cmd_history::HistoryCommand;
use crate::commands::cmd_pwd::PwdCommand;
use crate::commands::cmd_type::TypeCommand;
use crate::commands::Command;
use crate::utils::searcher::Searcher;
use crate::utils::Signal;
use crate::utils::Signal::CommandNotFound;

pub struct Factory;

impl Factory {
    pub fn is_builtin(command: &str) -> bool {
        match command {
            "cd"      => true,
            "echo"    => true,
            "exit"    => true,
            "history" => true,
            "pwd"     => true,
            "type"    => true,
            _ => false,
        }
    }

    pub fn complete(incomplete: &str) -> Vec<String> {
        use std::collections::HashSet;

        let builtins = ["cd", "echo", "exit", "history", "pwd", "type"];

        let mut result: Vec<String> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for b in builtins {
            if b.starts_with(incomplete) {
                seen.insert(b.to_string());
                result.push(b.to_string());
            }
        }

        let ext = Searcher::search_possible_executables_in_path(incomplete);
        for cmd in ext {
            if seen.insert(cmd.clone()) {
                result.push(cmd);
            }
        }

        result.sort();
        result
    }

    pub fn get(command: &str) -> Result<Box<dyn Command>, Signal> {
        match command {
            "cd"      => Ok(Box::new(CdCommand::new())),
            "echo"    => Ok(Box::new(EchoCommand::new())),
            "exit"    => Ok(Box::new(ExitCommand::new())),
            "history" => Ok(Box::new(HistoryCommand::new())),
            "pwd"     => Ok(Box::new(PwdCommand::new())),
            "type"    => Ok(Box::new(TypeCommand::new())),

            _ => {
                let Some(_) = Searcher::search_executable_in_path(command) else {
                    return Err(CommandNotFound { command: command.to_string() })
                };
                Ok(Box::new(ExternalCommand::new(command)))
            }
        }
    }
}