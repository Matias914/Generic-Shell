use crate::commands::cmd_cd::CdCommand;
use crate::commands::cmd_echo::EchoCommand;
use crate::commands::cmd_exit::ExitCommand;
use crate::commands::cmd_external::ExternalCommand;
use crate::commands::cmd_pwd::PwdCommand;
use crate::commands::cmd_type::TypeCommand;
use crate::commands::Command;
use crate::utils::searcher::ExecutableSearcher;
use crate::utils::Signal;
use crate::utils::Signal::CommandNotFound;

pub struct Factory;

impl Factory {
    pub fn is_builtin(command: &str) -> bool {
        match command {
            "cd"   => true,
            "echo" => true,
            "exit" => true,
            "pwd"  => true,
            "type" => true,
            _ => false,
        }
    }

    pub fn get_command(command: &str) -> Result<Box<dyn Command>, Signal> {
        match command {
            "cd"   => Ok(Box::new(CdCommand::new())),
            "echo" => Ok(Box::new(EchoCommand::new())),
            "exit" => Ok(Box::new(ExitCommand::new())),
            "pwd"  => Ok(Box::new(PwdCommand::new())),
            "type" => Ok(Box::new(TypeCommand::new())),

            _ => {
                let Some(_) = ExecutableSearcher::search_in_path(command) else {
                    return Err(CommandNotFound { command: command.to_string() })
                };
                Ok(Box::new(ExternalCommand::new(command)))
            }
        }
    }
}