#![warn(unused_crate_dependencies, clippy::unwrap_used)]

mod application;
mod key;
mod platform;
mod widgets;

pub mod views;

pub use application::{Application, Error};
pub use platform::Platform;

#[cfg(feature = "layer-shell")]
pub use views::{ExclusiveZone, KeyboardInput, Layer, LayerShell, layer_shell};

#[cfg(feature = "session-lock")]
pub use views::{SessionLock, session_lock};
