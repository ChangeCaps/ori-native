use crate::{Affine, NativeParent, NativeWidget, Platform};

pub trait HasTransform: Platform {
    type Transform: NativeTransform<Self>;
}

pub trait NativeTransform<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::Widget) -> Self;
    fn teardown(self, platform: &mut P);

    fn set_content_transform(&mut self, platform: &mut P, width: f32, height: f32, affine: Affine);
}
