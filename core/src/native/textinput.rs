use std::convert::Infallible;

use crate::{
    Font, Measure, NativeWidget, Platform, Unsupported, platform::unsupported, views::Newline,
};

pub trait NativeTextInput<P>: NativeWidget<P>
where
    P: Platform,
{
    fn build(platform: &mut P) -> Self;
    fn teardown(self, platform: &mut P);

    fn set_on_change(&mut self, platform: &mut P, on_change: impl Fn(String) + 'static);
    fn set_on_submit(&mut self, platform: &mut P, on_submit: impl Fn(String) + 'static);

    fn set_newline(&mut self, platform: &mut P, newline: Newline);
    fn set_accept_tab(&mut self, platform: &mut P, accept_tab: bool);

    fn set_font(&mut self, platform: &mut P, font: Font);
    fn set_text(&mut self, platform: &mut P, text: String);
    fn set_placeholder_font(&mut self, platform: &mut P, font: Font);
    fn set_placeholder_text(&mut self, platform: &mut P, text: String);

    fn get_layout(&mut self, platform: &mut P) -> impl Measure<P>;
}

impl<P> NativeTextInput<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P) -> Self {
        unsupported!("text input view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn set_on_change(&mut self, _platform: &mut P, _on_change: impl Fn(String) + 'static) {
        unreachable!()
    }

    fn set_on_submit(&mut self, _platform: &mut P, _on_submit: impl Fn(String) + 'static) {
        unreachable!()
    }

    fn set_newline(&mut self, _platform: &mut P, _newline: Newline) {
        unreachable!()
    }

    fn set_accept_tab(&mut self, _platform: &mut P, _accept_tab: bool) {
        unreachable!()
    }

    fn set_font(&mut self, _platform: &mut P, _font: Font) {
        unreachable!()
    }

    fn set_text(&mut self, _platform: &mut P, _text: String) {
        unreachable!()
    }

    fn set_placeholder_font(&mut self, _platform: &mut P, _font: Font) {
        unreachable!()
    }

    fn set_placeholder_text(&mut self, _platform: &mut P, _text: String) {
        unreachable!()
    }

    #[allow(refining_impl_trait)]
    fn get_layout(&mut self, _platform: &mut P) -> Infallible {
        unreachable!()
    }
}
