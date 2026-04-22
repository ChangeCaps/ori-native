use keyboard_types::{Key, Modifiers};

use crate::{NativeWidget, Platform, Unsupported, platform::unsupported};

/// A native widget that receives pointer input and focus.
pub trait NativePressable<P>: NativeWidget<P>
where
    P: Platform,
{
    /// Build a pressable.
    fn build(
        platform: &mut P,
        contents: P::WidgetRef,
        on_press: impl Fn(Press) + 'static,
        on_hover: impl Fn(bool) + 'static,
        on_focus: impl Fn(bool) + 'static,
    ) -> Self;

    /// Teardown the widget.
    fn teardown(self, platform: &mut P);

    /// Replace the contents;
    fn replace_contents(&mut self, platform: &mut P, contents: P::WidgetRef);

    /// Set the size of the contents.
    fn set_content_size(&mut self, platform: &mut P, width: f32, height: f32);

    /// Set the `on_key` callback.
    fn set_on_key(
        &mut self,
        platform: &mut P,
        on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    );
}

/// The state of a press.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Press {
    /// The pointer was depressed.
    Pressed,

    /// The pointer was released.
    Released,

    /// The press has been cancelled.
    Cancelled,
}

impl<P> NativePressable<P> for Unsupported
where
    P: Platform,
{
    fn build(
        _platform: &mut P,
        _contents: P::WidgetRef,
        _on_press: impl Fn(Press) + 'static,
        _on_hover: impl Fn(bool) + 'static,
        _on_focus: impl Fn(bool) + 'static,
    ) -> Self {
        unsupported!("pressable view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn replace_contents(&mut self, _platform: &mut P, _contents: P::WidgetRef) {
        unreachable!()
    }

    fn set_content_size(&mut self, _platform: &mut P, _width: f32, _height: f32) {
        unreachable!()
    }

    fn set_on_key(
        &mut self,
        _platform: &mut P,
        _on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    ) {
        unreachable!()
    }
}
