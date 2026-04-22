use jni::{EnvUnowned, jni_sig, jni_str, objects::JObject};
use ori_native_core::{NativeWidget, native::NativeMeasure};

use crate::{
    Platform,
    application::{GlobalState, WidgetEvent},
    platform::WidgetId,
};

pub struct Measure {
    id: WidgetId,
}

impl NativeWidget<Platform> for Measure {
    fn widget_ref(&self) -> WidgetId {
        self.id
    }
}

impl NativeMeasure<Platform> for Measure {
    fn build(
        platform: &mut Platform,
        contents: WidgetId,
        on_position_changed: impl Fn(f32, f32) + 'static,
    ) -> Self {
        let id = platform.next_id();

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createMeasure"),
                jni_sig!((long)),
                &[id.into()],
            )?
            .v()?;

            env.call_method(
                activity,
                jni_str!("measureSetContents"),
                jni_sig!((long, long)),
                &[id.into(), contents.into()],
            )?
            .v()
        });

        platform.add_handler(id, move |event| match event {
            WidgetEvent::Position(x, y) => on_position_changed(*x, *y),
            _ => unreachable!(),
        });

        Self { id }
    }

    fn teardown(self, platform: &mut Platform) {
        platform.remove_widget(self.id);
    }

    fn replace_contents(&mut self, platform: &mut Platform, child: WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("measureSetContents"),
                jni_sig!((long, long)),
                &[self.id.into(), child.into()],
            )?
            .v()
        });
    }

    fn set_content_size(&mut self, platform: &mut Platform, width: f32, height: f32) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("measureSetContentSize"),
                jni_sig!((long, float, float)),
                &[self.id.into(), width.into(), height.into()],
            )?
            .v()
        });
    }
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriActivity_measurePositionChanged<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    id: i64,
    x: f32,
    y: f32,
) {
    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Position(x, y),
    );
}
