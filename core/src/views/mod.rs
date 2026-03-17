//! Builtin views.

mod animate;
mod flex;
mod image;
mod pressable;
mod safearea;
mod scroll;
mod text;
mod textinput;
mod transform;
mod transition;
mod window;

pub use animate::{Animate, Animation, animate};
pub use flex::{Flex, column, flex, row};
pub use image::{Image, image};
pub use pressable::{PressState, Pressable, pressable};
pub use safearea::{SafeArea, safe_area};
pub use scroll::{Scroll, hscroll, vscroll};
pub use text::{Text, text};
pub use textinput::{Newline, TextInput, textinput};
pub use transform::{Transform, transform};
pub use transition::{
    Back, BackIn, BackInOut, Ease, Elastic, ElasticIn, Lerp, Linear, Transition, transition,
};
pub use window::{Window, WindowAttributes, WindowState, window};
