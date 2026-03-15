use jni::{
    EnvUnowned, jni_sig, jni_str,
    objects::{JObject, JString},
};
use ori_native_core::{Font, Measure, NativeWidget, native::NativeTextInput, views::Newline};

use crate::{
    Platform,
    application::{GlobalState, WidgetEvent},
    platform::WidgetId,
};

pub struct TextInput {
    id: WidgetId,
}

impl NativeWidget<Platform> for TextInput {
    fn widget(&self) -> &WidgetId {
        &self.id
    }
}

impl NativeTextInput<Platform> for TextInput {
    fn build(platform: &mut Platform) -> Self {
        let id = platform.next_id();

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createTextInput"),
                jni_sig!((long)),
                &[id.into()],
            )?
            .v()
        });

        Self { id }
    }

    fn teardown(self, platform: &mut Platform) {
        platform.remove_widget(self.id);
    }

    fn set_on_change(&mut self, platform: &mut Platform, on_change: impl Fn(String) + 'static) {
        platform.add_handler(self.id, move |event| match event {
            WidgetEvent::Change(text) => on_change(text.clone()),
            WidgetEvent::Submit(_) => {}
            _ => unreachable!(),
        });
    }

    fn set_on_submit(&mut self, platform: &mut Platform, on_submit: impl Fn(String) + 'static) {
        platform.add_handler(self.id, move |event| match event {
            WidgetEvent::Submit(text) => on_submit(text.clone()),
            WidgetEvent::Change(_) => {}
            _ => unreachable!(),
        });
    }

    fn set_newline(&mut self, platform: &mut Platform, newline: Newline) {
        let singleline = matches!(newline, Newline::None);

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("textInputSetSingleLine"),
                jni_sig!((long, boolean)),
                &[self.id.into(), singleline.into()],
            )?
            .v()
        });
    }

    fn set_accept_tab(&mut self, _platform: &mut Platform, _accept_tab: bool) {}

    fn set_font(&mut self, platform: &mut Platform, font: Font) {
        let _ = platform.jni(|env, activity| {
            let family = match font.family {
                Some(family) => env.new_string(family)?,
                None => JString::null(),
            };

            env.call_method(
                activity,
                jni_str!("textInputSetFont"),
                jni_sig!(
                    (
                        long,
                        float,
                        JString,
                        int,
                        int,
                        boolean,
                        boolean,
                        float,
                        float,
                        float,
                        float,
                    ) -> void
                ),
                &[
                    self.id.into(),
                    font.size.into(),
                    (&family).into(),
                    (font.weight.0 as i32).into(),
                    0i32.into(),
                    font.italic.into(),
                    font.striketrough.into(),
                    font.color.r.into(),
                    font.color.g.into(),
                    font.color.b.into(),
                    font.color.a.into(),
                ],
            )?
            .v()
        });
    }

    fn set_text(&mut self, platform: &mut Platform, text: String) {
        let _ = platform.jni(|env, activity| {
            let text = env.new_string(text)?;

            env.call_method(
                activity,
                jni_str!("textInputSetText"),
                jni_sig!((long, JString)),
                &[self.id.into(), (&text).into()],
            )?
            .v()
        });
    }

    fn set_placeholder_font(&mut self, platform: &mut Platform, font: Font) {
        let _ = platform.jni(|env, activity| {
            let family = match font.family {
                Some(family) => env.new_string(family)?,
                None => JString::null(),
            };

            env.call_method(
                activity,
                jni_str!("textInputSetPlaceholderFont"),
                jni_sig!(
                    (
                        long,
                        float,
                        JString,
                        int,
                        int,
                        boolean,
                        boolean,
                        float,
                        float,
                        float,
                        float,
                    ) -> void
                ),
                &[
                    self.id.into(),
                    font.size.into(),
                    (&family).into(),
                    (font.weight.0 as i32).into(),
                    0i32.into(),
                    font.italic.into(),
                    font.striketrough.into(),
                    font.color.r.into(),
                    font.color.g.into(),
                    font.color.b.into(),
                    font.color.a.into(),
                ],
            )?
            .v()
        });
    }

    fn set_placeholder_text(&mut self, platform: &mut Platform, text: String) {
        let _ = platform.jni(|env, activity| {
            let text = env.new_string(text)?;

            env.call_method(
                activity,
                jni_str!("textInputSetPlaceholderText"),
                jni_sig!((long, JString)),
                &[self.id.into(), (&text).into()],
            )?
            .v()
        });
    }

    fn get_layout(&mut self, _platform: &mut Platform) -> impl Measure<Platform> {
        TextInputLayout { id: self.id }
    }
}

pub struct TextInputLayout {
    id: WidgetId,
}

impl Measure<Platform> for TextInputLayout {
    fn measure(
        &mut self,
        platform: &mut Platform,
        _known_size: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        let height = platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("textInputMeasureHeight"),
                    jni_sig!((long) -> float),
                    &[self.id.into()],
                )?
                .f()
            })
            .unwrap_or(0.0);

        taffy::Size { width: 0.0, height }
    }
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriEditText_onChange<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    id: i64,
    text: JString<'local>,
) {
    let text = text.to_string();
    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Change(text),
    );
}

#[unsafe(no_mangle)]
extern "system" fn Java_ori_OriEditText_onSubmit<'local>(
    _env: EnvUnowned<'local>,
    _this: JObject<'local>,
    id: i64,
    text: JString<'local>,
) {
    let text = text.to_string();
    GlobalState::event(
        WidgetId::new(id as u64),
        WidgetEvent::Submit(text),
    );
}
