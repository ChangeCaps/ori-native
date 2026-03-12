use keyboard_types::{Key, Modifiers};

use crate::{NativeParent, NativeWidget, Platform, Unsupported, platform::unsupported};

pub trait NativePressable<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::Widget) -> Self;
    fn teardown(self, platform: &mut P);

    fn set_content_size(&mut self, platform: &mut P, width: f32, height: f32);

    fn set_on_press(&mut self, platform: &mut P, on_press: impl Fn(Press) + 'static);
    fn set_on_hover(&mut self, platform: &mut P, on_hover: impl Fn(bool) + 'static);
    fn set_on_focus(&mut self, platform: &mut P, on_focus: impl Fn(bool) + 'static);
    fn set_on_key(
        &mut self,
        platform: &mut P,
        on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Press {
    Pressed,
    Released,
    Cancelled,
}

impl<P> NativePressable<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P, _contents: &P::Widget) -> Self {
        unsupported!("pressable view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn set_content_size(&mut self, _platform: &mut P, _width: f32, _height: f32) {
        unreachable!()
    }

    fn set_on_press(&mut self, _platform: &mut P, _on_press: impl Fn(Press) + 'static) {
        unreachable!()
    }

    fn set_on_hover(&mut self, _platform: &mut P, _on_hover: impl Fn(bool) + 'static) {
        unreachable!()
    }

    fn set_on_focus(&mut self, _platform: &mut P, _on_focus: impl Fn(bool) + 'static) {
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
