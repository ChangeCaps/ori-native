#![warn(missing_docs, clippy::unwrap_used)]

//! Core implementation of `ori-native`.

mod context;
mod element;
mod input;
mod lifecycle;
mod platform;
mod style;
mod text;

pub mod native;
pub mod views;

pub use context::{BoxedEffect, Context, LayoutLeaf};
pub use element::{BoxedWidget, NativeParent, NativeWidget, Pod, PodMut, WidgetView};
pub use input::{Input, InputFilter, InputHandler, InputMessage, MatchKey};
pub use lifecycle::{AnimateRequest, LayoutRequest, Lifecycle};
pub use platform::Platform;
pub use style::{
    Affine, Align, AutoLength, Bordered, Color, Container, Direction, FlexContainer, Fract,
    Justify, Layoutable, Length, Overflow, Position, Shadow, Sizing,
};
pub use text::{Font, Stretch, TextSpan, Weight, Wrap};

pub use keyboard_types::{Key, Modifiers, NamedKey};
