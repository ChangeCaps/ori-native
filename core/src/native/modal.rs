use crate::{NativeParent, Platform, Unsupported, platform::unsupported};

pub trait NativeModal<P>: NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::Widget) -> Self;
    fn teardown(self, platform: &mut P);

    fn get_size(&self, platform: &mut P) -> (f32, f32);

    fn set_content_layout(&mut self, platform: &mut P, x: f32, y: f32, width: f32, height: f32);
}

impl<P> NativeModal<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P, _contents: &P::Widget) -> Self {
        unsupported!("modal")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn get_size(&self, _platform: &mut P) -> (f32, f32) {
        unreachable!()
    }

    fn set_content_layout(
        &mut self,
        _platform: &mut P,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
    ) {
        unreachable!()
    }
}
