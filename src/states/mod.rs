pub mod init;
pub mod running;

pub use init::InitState;

use crate::context::Context;
use crate::utils::Signal;

pub trait State {
    fn next(self: Box<Self>, word: &str) -> Result<Box<dyn State>, Signal>;
    fn end(&mut self, ctx: &mut Context);
}