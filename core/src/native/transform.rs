use crate::{Affine, NativeWidget, Platform, Unsupported, platform::unsupported};

/// A native view that transforms its contents.
pub trait NativeTransform<P>: NativeWidget<P>
where
    P: Platform,
{
    /// Build a transform widget.
    fn build(platform: &mut P, contents: P::WidgetRef) -> Self;

    /// Teardown the widget.
    fn teardown(self, platform: &mut P);

    /// Replace the contents.
    fn replace_contents(&mut self, platform: &mut P, contents: P::WidgetRef);

    /// Set the size and transform of the contents.
    fn set_content_transform(&mut self, platform: &mut P, width: f32, height: f32, affine: Affine);
}

impl<P> NativeTransform<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P, _contents: P::WidgetRef) -> Self {
        unsupported!("transform view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn replace_contents(&mut self, _platform: &mut P, _contents: P::WidgetRef) {
        unreachable!()
    }

    fn set_content_transform(
        &mut self,
        _platform: &mut P,
        _width: f32,
        _height: f32,
        _affine: Affine,
    ) {
        unreachable!()
    }
}
