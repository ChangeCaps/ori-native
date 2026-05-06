use std::convert::Infallible;

use crate::{Measurable, Platform, TextSpan, Unsupported, Wrap, platform::unsupported};

/// A native text widget.
pub trait NativeText<P>
where
    P: Platform,
{
    /// Build an empty text widget.
    fn build(platform: &mut P) -> Self;

    /// Teardown the widget.
    fn teardown(self, platform: &mut P);

    /// Get a reference to the widget.
    fn widget_ref(&self) -> P::WidgetRef;

    /// Set the text.
    fn set_text(
        &mut self,
        platform: &mut P,
        spans: Box<[TextSpan]>,
        text: String,
        wrap: Wrap,
    ) -> impl Measurable<P>;
}

impl<P> NativeText<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P) -> Self {
        unsupported!("text view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn widget_ref(&self) -> P::WidgetRef {
        unreachable!()
    }

    #[allow(refining_impl_trait)]
    fn set_text(
        &mut self,
        _platform: &mut P,
        _spans: Box<[TextSpan]>,
        _text: String,
        _wrap: Wrap,
    ) -> Infallible {
        unreachable!()
    }
}
