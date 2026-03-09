use std::time::Duration;

use keyboard_types::{Key, Modifiers};

use crate::{Platform, Unsupported, element::NativeParent, platform::unsupported};

pub trait NativeWindow<P>: NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::Widget) -> Self;

    fn teardown(self, platform: &mut P);

    fn get_size(&self) -> (u32, u32);
    fn get_preferred_size(&self) -> (Option<u32>, Option<u32>);

    fn set_on_animation_frame(&mut self, on_frame: impl Fn(Duration) + 'static);
    fn set_on_resize(&mut self, on_resize: impl Fn() + 'static);
    fn set_on_close_requested(&mut self, on_close_requested: impl Fn() + 'static);
    fn set_on_key(&mut self, on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static);

    fn start_animating(&mut self);
    fn stop_animating(&mut self);

    fn set_content_size(&mut self, platform: &mut P, width: f32, height: f32);

    fn set_title(&mut self, title: String);
    fn set_min_size(&mut self, width: u32, height: u32);
    fn set_size(&mut self, width: u32, height: u32);
    fn set_resizable(&mut self, resizable: bool);
}

impl<P> NativeWindow<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P, _contents: &P::Widget) -> Self {
        unsupported!("window view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn get_size(&self) -> (u32, u32) {
        unreachable!()
    }

    fn get_preferred_size(&self) -> (Option<u32>, Option<u32>) {
        unreachable!()
    }

    fn set_on_animation_frame(&mut self, _on_frame: impl Fn(Duration) + 'static) {
        unreachable!()
    }

    fn set_on_resize(&mut self, _on_resize: impl Fn() + 'static) {
        unreachable!()
    }

    fn set_on_close_requested(&mut self, _on_close_requested: impl Fn() + 'static) {
        unreachable!()
    }

    fn set_on_key(&mut self, _on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static) {
        unreachable!()
    }

    fn start_animating(&mut self) {
        unreachable!()
    }

    fn stop_animating(&mut self) {
        unreachable!()
    }

    fn set_content_size(&mut self, _platform: &mut P, _width: f32, _height: f32) {
        unreachable!()
    }

    fn set_title(&mut self, _title: String) {
        unreachable!()
    }

    fn set_min_size(&mut self, _width: u32, _height: u32) {
        unreachable!()
    }

    fn set_size(&mut self, _width: u32, _height: u32) {
        unreachable!()
    }

    fn set_resizable(&mut self, _resizable: bool) {
        unreachable!()
    }
}
