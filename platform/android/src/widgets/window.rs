use std::time::Duration;

use jni::{EnvUnowned, jni_sig, jni_str, objects::JObject};
use ori_native_core::{Key, Modifiers, NativeParent, native::NativeWindow};

use crate::{
    Platform,
    application::{ACTIVITY, Event},
    platform::WidgetId,
};

pub struct Window {}

impl NativeParent<Platform> for Window {
    fn replace_child(&mut self, platform: &mut Platform, _index: usize, child: &WidgetId) {
        platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowSetContents"),
                    jni_sig!((long)),
                    &[child.into()],
                )?
                .v()
            })
            .unwrap();
    }
}

impl NativeWindow<Platform> for Window {
    fn build(platform: &mut Platform, contents: &WidgetId) -> Self {
        platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowSetContents"),
                    jni_sig!((long)),
                    &[contents.into()],
                )?
                .v()
            })
            .unwrap();

        Self {}
    }

    fn teardown(self, _platform: &mut Platform) {}

    fn get_size(&self, platform: &mut Platform) -> (u32, u32) {
        let width = platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowGetWidth"),
                    jni_sig!(() -> int),
                    &[],
                )?
                .i()
            })
            .unwrap();

        let height = platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowGetHeight"),
                    jni_sig!(() -> int),
                    &[],
                )?
                .i()
            })
            .unwrap();

        (width as u32, height as u32)
    }

    fn get_preferred_size(&self, platform: &mut Platform) -> (Option<u32>, Option<u32>) {
        let (width, height) = self.get_size(platform);

        (Some(width), Some(height))
    }

    fn set_on_animation_frame(
        &mut self,
        platform: &mut Platform,
        on_frame: impl Fn(Duration) + 'static,
    ) {
        platform.set_on_animation_frame(on_frame);
    }

    fn set_on_resize(&mut self, _platform: &mut Platform, _on_resize: impl Fn() + 'static) {}

    fn set_on_close_requested(
        &mut self,
        _platform: &mut Platform,
        _on_close_requested: impl Fn() + 'static,
    ) {
    }

    fn set_on_key(
        &mut self,
        _platform: &mut Platform,
        _on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    ) {
    }

    fn start_animating(&mut self, platform: &mut Platform) {
        platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowStartAnimating"),
                    jni_sig!(()),
                    &[],
                )?
                .v()
            })
            .unwrap();
    }

    fn stop_animating(&mut self, platform: &mut Platform) {
        platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowStopAnimating"),
                    jni_sig!(()),
                    &[],
                )?
                .v()
            })
            .unwrap();
    }

    fn set_content_size(&mut self, platform: &mut Platform, width: f32, height: f32) {
        platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("windowSetContentSize"),
                    jni_sig!((float, float)),
                    &[width.into(), height.into()],
                )?
                .v()
            })
            .unwrap();
    }

    fn set_title(&mut self, _platform: &mut Platform, _title: String) {}
    fn set_min_size(&mut self, _platform: &mut Platform, _width: u32, _height: u32) {}
    fn set_size(&mut self, _platform: &mut Platform, _width: u32, _height: u32) {}
    fn set_resizable(&mut self, _platform: &mut Platform, _resizable: bool) {}
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriActivity_onAnimationFrame<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    duration_nanos: i64,
) {
    if let Some(activity) = ACTIVITY.get() {
        let duration = Duration::from_nanos(duration_nanos as u64);
        let _ = activity.sender.send(Event::Frame(duration));
    }
}
