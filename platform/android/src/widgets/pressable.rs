use jni::{EnvUnowned, jni_sig, jni_str, objects::JObject};
use ori_native_core::{
    Key, Modifiers, NativeWidget, Pointer, PressableEvent, native::NativePressable,
};

use crate::{
    Platform,
    application::{GlobalState, WidgetEvent},
    platform::WidgetId,
};

pub struct Pressable {
    id: WidgetId,
}

impl NativeWidget<Platform> for Pressable {
    fn widget_ref(&self) -> WidgetId {
        self.id
    }
}

impl NativePressable<Platform> for Pressable {
    fn build(
        platform: &mut Platform,
        contents: WidgetId,
        on_event: impl Fn(PressableEvent) + 'static,
    ) -> Self {
        let id = platform.next_id();

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createPressable"),
                jni_sig!((long)),
                &[id.into()],
            )?
            .v()?;

            env.call_method(
                activity,
                jni_str!("pressableSetContents"),
                jni_sig!((long, long)),
                &[id.into(), contents.into()],
            )?
            .v()
        });

        platform.add_handler(id, move |event| match event {
            WidgetEvent::Press(evnet) => on_event(*evnet),
            _ => unreachable!(),
        });

        Self { id }
    }

    fn teardown(self, platform: &mut Platform) {
        platform.remove_widget(self.id);
    }

    fn replace_contents(&mut self, platform: &mut Platform, contents: WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("pressableSetContents"),
                jni_sig!((long, long)),
                &[self.id.into(), contents.into()],
            )?
            .v()
        });
    }

    fn set_content_size(&mut self, platform: &mut Platform, width: f32, height: f32) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("pressableSetContentSize"),
                jni_sig!((long, float, float)),
                &[self.id.into(), width.into(), height.into()],
            )?
            .v()
        });
    }

    fn set_on_key(
        &mut self,
        _platform: &mut Platform,
        _on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    ) {
    }
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriPressable_onPress<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    id: i64,
    state: i32,
    x: f32,
    y: f32,
) -> bool {
    let pointer = Pointer { x, y };

    let event = match state {
        0 => PressableEvent::Pressed(pointer),
        1 => PressableEvent::Released(pointer),
        2 => PressableEvent::Cancelled(pointer),
        _ => return false,
    };

    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Press(event),
    );

    true
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriPressable_onMove<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    id: i64,
    x: f32,
    y: f32,
) -> bool {
    let pointer = Pointer { x, y };
    let event = PressableEvent::Moved(pointer);

    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Press(event),
    );

    true
}
