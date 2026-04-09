use crate::{Affine, NativeParent, NativeWidget, Platform, Unsupported, platform::unsupported};

pub trait NativeTransform<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::WidgetRef) -> Self;
    fn teardown(self, platform: &mut P);

    fn set_content_transform(&mut self, platform: &mut P, width: f32, height: f32, affine: Affine);
}

impl<P> NativeTransform<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P, _contents: &P::WidgetRef) -> Self {
        unsupported!("transform view")
    }

    fn teardown(self, _platform: &mut P) {
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
