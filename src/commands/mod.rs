pub mod cmd_echo;
mod cmd_exit;
mod cmd_type;
mod cmd_external;
pub mod factory;
pub mod cmd_pwd;
mod cmd_cd;

use crate::context::Context;

pub trait Command {
    fn execute(&mut self, ctx: &mut Context);
    fn add_argument(&mut self, arg: &str);
}