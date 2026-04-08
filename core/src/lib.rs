#![warn(missing_docs, unused_crate_dependencies, clippy::unwrap_used)]

//! Core implementation of `ori-native`.

mod context;
mod element;
mod input;
mod layout;
mod lifecycle;
mod platform;
mod safearea;
mod style;
mod text;

pub mod native;
pub mod views;

pub use context::{BoxedEffect, Context};
pub use element::{
    BoxedWidget, NativeParent, NativeWidget, Pod, PodMut, WidgetView, WidgetViewSeq,
};
pub use input::{Input, InputFilter, InputHandler, InputMessage, MatchKey};
pub use layout::{Allocation, AvailableSpace, LayoutNode, LayoutTree, Measurable};
pub use lifecycle::{AnimateRequest, LayoutRequest, Lifecycle, ModalRequest};
pub use platform::{Platform, Unsupported};
pub use safearea::SafeAreaInsets;
pub use style::{
    Affine, Align, Border, BorderStyle, Color, Corners, Direction, FlexContainer, FlexStyle, Fract,
    Justify, Layout, LayoutStyle, Length, NavigationBar, Overflow, Padding, Position, Shadow,
    Sides, Size, Sizing, StatusBar,
};
pub use text::{Font, Stretch, TextSpan, Weight, Wrap};

pub use keyboard_types::{Key, Modifiers, NamedKey};
