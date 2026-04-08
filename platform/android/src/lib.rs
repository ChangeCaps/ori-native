#![warn(unused_crate_dependencies, clippy::unwrap_used)]

mod application;
mod entry;
mod log;
mod platform;
mod widgets;

pub use application::{Application, Error};
pub use platform::Platform;

#[doc(hidden)]
pub use entry::entry;
