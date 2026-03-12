#![warn(clippy::unwrap_used)]

mod app;
mod view;

pub use app::App;
pub use view::{Effect, View};

pub use ori::*;
pub use ori_native_core::*;

pub use ori_native_macro::main;

#[cfg(target_os = "linux")]
pub use ori_native_gtk4 as platform;

#[cfg(target_os = "android")]
pub use ori_native_android as platform;

pub type Platform = platform::Platform;
pub type Context = ori_native_core::Context<Platform>;
pub type Element = <Context as ori::Base>::Element;
pub type Error = platform::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub mod views {
    pub use ori::views::*;
    pub use ori_native_core::views::*;

    #[cfg(feature = "layer-shell")]
    pub use ori_native_gtk4::{ExclusiveZone, KeyboardInput, Layer, LayerShell, layer_shell};

    #[cfg(feature = "session-lock")]
    pub use ori_native_gtk4::{SessionLock, session_lock};
}

pub mod prelude {
    pub use crate::{
        Action, Align, App, AutoLength, Bordered, BuildMarker, BuildView, Color, Container,
        Context, Effect, Element, FlexContainer, Font, Fract, Justify, Key, Keyed, Layoutable,
        Length, Message, Modifiers, NamedKey, Overflow, Position, Proxy, Shadow, Sizing, Stretch,
        View, Weight, Wrap, keyed, views::*,
    };

    #[allow(unused_imports)]
    #[cfg(target_os = "linux")]
    pub use crate::platform::views::*;

    pub use tracing::{debug, error, info, trace, warn};
}
