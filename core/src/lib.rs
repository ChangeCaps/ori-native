#![warn(missing_docs, clippy::unwrap_used)]

//! Core implementation of `ori-native`.

mod context;
mod element;
mod input;
mod lifecycle;
mod platform;
mod safearea;
mod style;
mod text;

pub mod native;
pub mod views;

pub use context::{BoxedEffect, Context, Measure};
pub use element::{
    BoxedWidget, NativeParent, NativeWidget, Pod, PodMut, WidgetView, WidgetViewSeq,
};
pub use input::{Input, InputFilter, InputHandler, InputMessage, MatchKey};
pub use lifecycle::{AnimateRequest, LayoutRequest, Lifecycle};
pub use platform::{Platform, Unsupported};
pub use safearea::SafeAreaInsets;
pub use style::{
    Affine, Align, AutoLength, Border, Color, Container, Direction, FlexContainer, Fract, Justify,
    Layout, Length, NavigationBar, Overflow, Position, Shadow, Sizing, StatusBar,
};
pub use text::{Font, Stretch, TextSpan, Weight, Wrap};

pub use keyboard_types::{Key, Modifiers, NamedKey};
