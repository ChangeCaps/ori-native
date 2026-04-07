use jni::{EnvUnowned, jni_sig, jni_str, objects::JObject};
use ori_native_core::{Direction, NativeParent, NativeWidget, native::NativeScroll};

use crate::{
    Platform,
    application::{GlobalState, WidgetEvent},
    platform::WidgetId,
};

pub struct Scroll {
    id: WidgetId,
}

impl NativeWidget<Platform> for Scroll {
    fn widget(&self) -> &WidgetId {
        &self.id
    }
}

impl NativeParent<Platform> for Scroll {
    fn replace_child(&mut self, platform: &mut Platform, _index: usize, child: &WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("scrollSetContents"),
                jni_sig!((long, long)),
                &[self.id.into(), child.into()],
            )?
            .v()
        });
    }
}

impl NativeScroll<Platform> for Scroll {
    fn build(platform: &mut Platform, contents: &WidgetId) -> Self {
        let id = platform.next_id();

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createScroll"),
                jni_sig!((long)),
                &[id.into()],
            )?
            .v()?;

            env.call_method(
                activity,
                jni_str!("scrollSetContents"),
                jni_sig!((long, long)),
                &[id.into(), contents.into()],
            )?
            .v()
        });

        Self { id }
    }

    fn teardown(self, platform: &mut Platform) {
        platform.remove_widget(self.id);
    }

    fn set_on_scroll(&mut self, platform: &mut Platform, on_scroll: impl Fn(f32, f32) + 'static) {
        platform.add_handler(self.id, move |event| match event {
            WidgetEvent::Scroll(x, y) => on_scroll(*x, *y),
            _ => unreachable!(),
        });
    }

    fn set_content_size(&mut self, platform: &mut Platform, width: f32, height: f32) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("scrollSetContentSize"),
                jni_sig!((long, float, float)),
                &[self.id.into(), width.into(), height.into()],
            )?
            .v()
        });
    }

    fn set_content_layout(
        &mut self,
        platform: &mut Platform,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("scrollSetContentLayout"),
                jni_sig!((long, float, float, float, float)),
                &[
                    self.id.into(),
                    x.into(),
                    y.into(),
                    width.into(),
                    height.into(),
                ],
            )?
            .v()
        });
    }

    fn set_direction(&mut self, platform: &mut Platform, direction: Direction) {
        let is_vertical = matches!(direction, Direction::Column);

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("scrollSetVertical"),
                jni_sig!((long, boolean)),
                &[self.id.into(), is_vertical.into()],
            )?
            .v()
        });
    }
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriActivity_onScrolled<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    id: i64,
    x: f32,
    y: f32,
) {
    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Scroll(x, y),
    );
}
