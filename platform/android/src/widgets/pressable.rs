use jni::{EnvUnowned, jni_sig, jni_str, objects::JObject};
use ori_native_core::{
    Key, Modifiers, NativeParent, NativeWidget,
    native::{NativePressable, Press},
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
    fn widget_ref(&self) -> &WidgetId {
        &self.id
    }
}

impl NativeParent<Platform> for Pressable {
    fn replace_child(&mut self, platform: &mut Platform, _index: usize, child: &WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("pressableSetContents"),
                jni_sig!((long, long)),
                &[self.id.into(), child.into()],
            )?
            .v()
        });
    }
}

impl NativePressable<Platform> for Pressable {
    fn build(platform: &mut Platform, contents: &WidgetId) -> Self {
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

        Self { id }
    }

    fn teardown(self, platform: &mut Platform) {
        platform.remove_widget(self.id);
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

    fn set_on_press(&mut self, platform: &mut Platform, on_press: impl Fn(Press) + 'static) {
        platform.add_handler(self.id, move |event| match event {
            WidgetEvent::Press(press) => on_press(*press),
            _ => unreachable!(),
        });
    }

    fn set_on_hover(&mut self, _platform: &mut Platform, _on_hover: impl Fn(bool) + 'static) {}

    fn set_on_focus(&mut self, _platform: &mut Platform, _on_focus: impl Fn(bool) + 'static) {}

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
) -> bool {
    let state = match state {
        0 => Press::Pressed,
        1 => Press::Released,
        2 => Press::Cancelled,
        _ => return false,
    };

    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Press(state),
    );

    true
}
