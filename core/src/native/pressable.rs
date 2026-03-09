use keyboard_types::{Key, Modifiers};

use crate::{NativeParent, NativeWidget, Platform, Unsupported, platform::unsupported};

pub trait NativePressable<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    fn build(plaform: &mut P, contents: &P::Widget) -> Self;
    fn teardown(self, plaform: &mut P);

    fn set_content_size(&mut self, plaform: &mut P, width: f32, height: f32);

    fn set_on_press(&mut self, on_press: impl Fn(Press) + 'static);
    fn set_on_hover(&mut self, on_hover: impl Fn(bool) + 'static);
    fn set_on_focus(&mut self, on_focus: impl Fn(bool) + 'static);
    fn set_on_key(&mut self, on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static);
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
    fn build(_plaform: &mut P, _contents: &P::Widget) -> Self {
        unsupported!("pressable view")
    }

    fn teardown(self, _plaform: &mut P) {
        unreachable!()
    }

    fn set_content_size(&mut self, _plaform: &mut P, _width: f32, _height: f32) {
        unreachable!()
    }

    fn set_on_press(&mut self, _on_press: impl Fn(Press) + 'static) {
        unreachable!()
    }

    fn set_on_hover(&mut self, _on_hover: impl Fn(bool) + 'static) {
        unreachable!()
    }

    fn set_on_focus(&mut self, _on_focus: impl Fn(bool) + 'static) {
        unreachable!()
    }

    fn set_on_key(&mut self, _on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static) {
        unreachable!()
    }
}
