use std::time::Duration;

use keyboard_types::{Key, Modifiers};

use crate::{
    NavigationBar, Platform, StatusBar, Unsupported, element::NativeParent, platform::unsupported,
};

pub trait NativeWindow<P>: NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P, contents: &P::Widget) -> Self;

    fn teardown(self, platform: &mut P);

    fn get_size(&self, platform: &mut P) -> (u32, u32);
    fn get_preferred_size(&self, platform: &mut P) -> (Option<u32>, Option<u32>);

    fn set_on_animation_frame(&mut self, platform: &mut P, on_frame: impl Fn(Duration) + 'static);
    fn set_on_resize(&mut self, platform: &mut P, on_resize: impl Fn() + 'static);
    fn set_on_close_requested(&mut self, platform: &mut P, on_close_requested: impl Fn() + 'static);
    fn set_on_key(
        &mut self,
        platform: &mut P,
        on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    );

    fn start_animating(&mut self, platform: &mut P);
    fn stop_animating(&mut self, platform: &mut P);

    fn set_content_layout(&mut self, platform: &mut P, x: f32, y: f32, width: f32, height: f32);

    fn set_title(&mut self, platform: &mut P, title: String);
    fn set_min_size(&mut self, platform: &mut P, width: u32, height: u32);
    fn set_size(&mut self, platform: &mut P, width: u32, height: u32);
    fn set_resizable(&mut self, platform: &mut P, resizable: bool);

    fn set_status_bar(&mut self, platform: &mut P, bar: StatusBar);
    fn set_navigation_bar(&mut self, platform: &mut P, bar: NavigationBar);
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

    fn get_size(&self, _platform: &mut P) -> (u32, u32) {
        unreachable!()
    }

    fn get_preferred_size(&self, _platform: &mut P) -> (Option<u32>, Option<u32>) {
        unreachable!()
    }

    fn set_on_animation_frame(
        &mut self,
        _platform: &mut P,
        _on_frame: impl Fn(Duration) + 'static,
    ) {
        unreachable!()
    }

    fn set_on_resize(&mut self, _platform: &mut P, _on_resize: impl Fn() + 'static) {
        unreachable!()
    }

    fn set_on_close_requested(
        &mut self,
        _platform: &mut P,
        _on_close_requested: impl Fn() + 'static,
    ) {
        unreachable!()
    }

    fn set_on_key(
        &mut self,
        _platform: &mut P,
        _on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    ) {
        unreachable!()
    }

    fn start_animating(&mut self, _platform: &mut P) {
        unreachable!()
    }

    fn stop_animating(&mut self, _platform: &mut P) {
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

    fn set_title(&mut self, _platform: &mut P, _title: String) {
        unreachable!()
    }

    fn set_min_size(&mut self, _platform: &mut P, _width: u32, _height: u32) {
        unreachable!()
    }

    fn set_size(&mut self, _platform: &mut P, _width: u32, _height: u32) {
        unreachable!()
    }

    fn set_resizable(&mut self, _platform: &mut P, _resizable: bool) {
        unreachable!()
    }

    fn set_status_bar(&mut self, _platform: &mut P, _bar: StatusBar) {
        unreachable!()
    }

    fn set_navigation_bar(&mut self, _platform: &mut P, _bar: NavigationBar) {
        unreachable!()
    }
}
