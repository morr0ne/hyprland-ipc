mod dispatch;
mod error;
mod event;

pub use dispatch::Dispatcher;
pub use error::{Error, Result};
pub use event::{Event, EventListener};
