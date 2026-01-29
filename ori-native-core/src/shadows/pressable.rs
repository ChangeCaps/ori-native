use crate::{
    Context, Shadow,
    native::{HasPressable, NativePressable, Press},
};

pub struct PressableShadow<P, S>
where
    P: HasPressable,
{
    pub pressable: P::Pressable,
    pub contents:  S,
}

impl<P, S> PressableShadow<P, S>
where
    P: HasPressable,
    S: Shadow<P>,
{
    pub fn new(cx: &mut Context<P>, contents: S) -> Self {
        Self {
            pressable: P::Pressable::build(&mut cx.platform, contents.widget()),
            contents,
        }
    }

    pub fn set_on_press(&mut self, on_press: impl Fn(Press) + 'static) {
        self.pressable.set_on_press(on_press);
    }

    pub fn set_on_hover(&mut self, on_hover: impl Fn(bool) + 'static) {
        self.pressable.set_on_hover(on_hover);
    }

    pub fn set_on_focus(&mut self, on_focus: impl Fn(bool) + 'static) {
        self.pressable.set_on_focus(on_focus);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.pressable.set_size(width, height);
    }
}

impl<P, S> Shadow<P> for PressableShadow<P, S>
where
    P: HasPressable,
    S: Shadow<P>,
{
    fn widget(&self) -> &P::Widget {
        self.pressable.widget()
    }
}
