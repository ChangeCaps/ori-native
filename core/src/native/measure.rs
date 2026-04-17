use crate::{NativeParent, NativeWidget, Platform, Unsupported, platform::unsupported};

/// A native widget that measures its global position.
pub trait NativeMeasure<P>: NativeParent<P> + NativeWidget<P>
where
    P: Platform,
{
    /// Build a measure, with callback called when global position changes.
    fn build(
        platform: &mut P,
        contents: &P::WidgetRef,
        on_position_changed: impl Fn(f32, f32) + 'static,
    ) -> Self;

    /// Teardown the widget.
    fn teardown(self, platform: &mut P);

    /// Set the size of the contents.
    fn set_content_size(&mut self, platform: &mut P, width: f32, height: f32);
}

impl<P> NativeMeasure<P> for Unsupported
where
    P: Platform,
{
    fn build(
        _platform: &mut P,
        _contents: &P::WidgetRef,
        _on_position_changed: impl Fn(f32, f32) + 'static,
    ) -> Self {
        unsupported!("measure view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn set_content_size(&mut self, _platform: &mut P, _width: f32, _height: f32) {
        unreachable!()
    }
}
