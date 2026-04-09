use crate::{NativeParent, NativeWidget, Platform, Unsupported, platform::unsupported};

pub trait NativeMeasure<P>: NativeParent<P> + NativeWidget<P>
where
    P: Platform,
{
    fn build(
        platform: &mut P,
        contents: &P::WidgetRef,
        on_position_changed: impl Fn(f32, f32) + 'static,
    ) -> Self;
    fn teardown(self, platform: &mut P);

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
