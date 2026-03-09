use std::convert::Infallible;

use crate::{
    LayoutLeaf, NativeWidget, Platform, TextSpan, Unsupported, Wrap, platform::unsupported,
};

pub trait NativeText<P>: NativeWidget<P> + Sized
where
    P: Platform,
{
    type Layout: LayoutLeaf<P>;

    fn build(
        platform: &mut P,
        spans: Box<[TextSpan]>,
        text: String,
        wrap: Wrap,
    ) -> (Self, Self::Layout);

    fn teardown(self, platform: &mut P);

    fn set_text(&mut self, spans: Box<[TextSpan]>, text: String, wrap: Wrap) -> Self::Layout;
}

impl<P> NativeText<P> for Unsupported
where
    P: Platform,
{
    type Layout = Infallible;

    fn build(
        _platform: &mut P,
        _spans: Box<[TextSpan]>,
        _text: String,
        _wrap: Wrap,
    ) -> (Self, Self::Layout) {
        unsupported!("text view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn set_text(&mut self, _spans: Box<[TextSpan]>, _text: String, _wrap: Wrap) -> Self::Layout {
        unreachable!()
    }
}
