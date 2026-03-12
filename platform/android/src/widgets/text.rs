use jni::{jni_sig, jni_str, objects::JString};
use ori_native_core::{LayoutLeaf, NativeWidget, TextSpan, Wrap, native::NativeText};

use crate::{Platform, platform::WidgetId};

pub struct Text {
    id: WidgetId,
}

impl NativeWidget<Platform> for Text {
    fn widget(&self) -> &WidgetId {
        &self.id
    }
}

impl NativeText<Platform> for Text {
    type Layout = TextLayout;

    fn build(platform: &mut Platform) -> Self {
        let id = platform.next_id();

        platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("createText"),
                    jni_sig!((long) -> void),
                    &[id.into()],
                )?
                .v()
            })
            .unwrap();

        Self { id }
    }

    fn teardown(self, platform: &mut Platform) {
        platform.remove_widget(self.id);
    }

    fn set_text(
        &mut self,
        platform: &mut Platform,
        spans: Box<[TextSpan]>,
        text: String,
        wrap: Wrap,
    ) -> Self::Layout {
        platform
            .jni(|env, activity| {
                let text = env.new_string(text)?;

                let wrap = match wrap {
                    Wrap::None => 3,
                    Wrap::Char => 1,
                    Wrap::Word => 2,
                };

                env.call_method(
                    activity,
                    jni_str!("textSetText"),
                    jni_sig!((long, JString, int) -> void),
                    &[self.id.into(), (&text).into(), wrap.into()],
                )?
                .v()?;

                for span in spans {
                    let family = match span.font.family {
                        Some(family) => env.new_string(family)?,
                        None => JString::null(),
                    };

                    env.call_method(
                        activity,
                        jni_str!("textSetSpan"),
                        jni_sig!((
                            long,
                            int,
                            int,
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
                        ) -> void),
                        &[
                            self.id.into(),
                            (span.range.start as i32).into(),
                            (span.range.end as i32).into(),
                            span.font.size.into(),
                            (&family).into(),
                            (span.font.weight.0 as i32).into(),
                            0i32.into(),
                            span.font.italic.into(),
                            span.font.striketrough.into(),
                            span.font.color.r.into(),
                            span.font.color.g.into(),
                            span.font.color.b.into(),
                            span.font.color.a.into(),
                        ],
                    )?
                    .v()?;
                }

                Ok::<_, jni::errors::Error>(())
            })
            .unwrap();

        TextLayout { id: self.id }
    }
}

pub struct TextLayout {
    id: WidgetId,
}

impl LayoutLeaf<Platform> for TextLayout {
    fn measure(
        &mut self,
        platform: &mut Platform,
        _known_size: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        let width = platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("textMeasureWidth"),
                    jni_sig!((long, float) -> float),
                    &[self.id.into(), 10000.0f32.into()],
                )?
                .f()
            })
            .unwrap();

        let height = platform
            .jni(|env, activity| {
                env.call_method(
                    activity,
                    jni_str!("textMeasureHeight"),
                    jni_sig!((long, float) -> float),
                    &[self.id.into(), 10000.0f32.into()],
                )?
                .f()
            })
            .unwrap();

        taffy::Size {
            width: width + 1.0,
            height,
        }
    }
}
