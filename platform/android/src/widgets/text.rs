use jni::{jni_sig, jni_str, objects::JString};
use ori_native_core::{
    AvailableSpace, Measure, NativeWidget, Size, TextSpan, Wrap, native::NativeText,
};

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

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createText"),
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

    fn set_text(
        &mut self,
        platform: &mut Platform,
        spans: Box<[TextSpan]>,
        text: String,
        wrap: Wrap,
    ) -> Self::Layout {
        let _ = platform.jni(|env, activity| {
            let jstring = env.new_string(&text)?;

            let wrap = match wrap {
                Wrap::None => 3,
                Wrap::Char => 1,
                Wrap::Word => 2,
            };

            env.call_method(
                activity,
                jni_str!("textSetText"),
                jni_sig!((long, JString, int)),
                &[self.id.into(), (&jstring).into(), wrap.into()],
            )?
            .v()?;

            for span in spans {
                let family = match span.font.family {
                    Some(family) => env.new_string(family)?,
                    None => JString::null(),
                };

                let start = text
                    .char_indices()
                    .enumerate()
                    .find(|(_, (offset, _))| *offset == span.range.start)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                let end = text
                    .char_indices()
                    .enumerate()
                    .find(|(_, (offset, _))| *offset == span.range.end)
                    .map(|(i, _)| i)
                    .unwrap_or_else(|| text.chars().count());

                env.call_method(
                    activity,
                    jni_str!("textSetSpan"),
                    jni_sig!(
                        (
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
                        ) -> void
                    ),
                    &[
                        self.id.into(),
                        (start as i32).into(),
                        (end as i32).into(),
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
        });

        TextLayout {
            id:    self.id,
            cache: Vec::new(),
        }
    }
}

pub struct TextLayout {
    id:    WidgetId,
    cache: Vec<CachedSize>,
}

struct CachedSize {
    size:            Size<f32>,
    known_size:      Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
}

impl Measure<Platform> for TextLayout {
    fn measure(
        &mut self,
        platform: &mut Platform,
        known_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32> {
        for cached_size in self.cache.iter() {
            if cached_size.known_size == known_size
                && cached_size.available_space == available_space
            {
                return cached_size.size;
            }
        }

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
            .unwrap_or(0.0);

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
            .unwrap_or(0.0);

        let size = Size {
            width: width + 1.0,
            height,
        };

        self.cache.push(CachedSize {
            size,
            known_size,
            available_space,
        });

        size
    }
}
