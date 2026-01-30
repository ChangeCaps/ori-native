use crate::{NativeWidget, Platform};

pub trait HasPressable: Platform {
    type Pressable: NativePressable<Self>;
}

pub trait NativePressable<P>: NativeWidget<P>
where
    P: Platform,
{
    fn build(plaform: &mut P, contents: &P::Widget) -> Self;
    fn teardown(self, plaform: &mut P);

    fn set_size(&mut self, width: f32, height: f32);
    fn set_on_press(&mut self, on_press: impl Fn(Press) + 'static);
    fn set_on_hover(&mut self, on_hover: impl Fn(bool) + 'static);
    fn set_on_focus(&mut self, on_focus: impl Fn(bool) + 'static);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Press {
    Pressed,
    Released,
    Cancelled,
}
