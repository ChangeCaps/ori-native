use crate::{
    Direction, NativeWidget, Platform, Unsupported, element::NativeParent, platform::unsupported,
};

pub trait NativeScroll<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::Widget) -> Self;
    fn teardown(self, platform: &mut P);

    fn set_content_size(&mut self, platform: &mut P, width: f32, height: f32);
    fn set_content_layout(&mut self, platform: &mut P, x: f32, y: f32, width: f32, height: f32);

    fn set_direction(&mut self, platform: &mut P, direction: Direction);
}

impl<P> NativeScroll<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P, _contents: &P::Widget) -> Self {
        unsupported!("scroll view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn set_content_size(&mut self, _platform: &mut P, _width: f32, _height: f32) {
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

    fn set_direction(&mut self, _platform: &mut P, _direction: Direction) {
        unreachable!()
    }
}
