pub use crate::prelude::*;

mod connect;
pub use connect::*;
mod handshake;
pub use handshake::*;
mod login;
pub use login::*;
mod network;
pub use network::*;
mod status;
pub use status::*;

pub type Task = Pin<Box<dyn Future<Output = Result<(), ()>> + Send + Sync + 'static>>;
