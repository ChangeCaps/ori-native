use jni::{jni_sig, jni_str};
use ori_native_core::{Affine, NativeParent, NativeWidget, native::NativeTransform};

use crate::{Platform, platform::WidgetId};

pub struct Transform {
    id: WidgetId,
}

impl NativeWidget<Platform> for Transform {
    fn widget_ref(&self) -> &WidgetId {
        &self.id
    }
}

impl NativeParent<Platform> for Transform {
    fn replace_child(&mut self, platform: &mut Platform, _index: usize, child: &WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("transformSetContents"),
                jni_sig!((long, long)),
                &[self.id.into(), child.into()],
            )?
            .v()
        });
    }
}

impl NativeTransform<Platform> for Transform {
    fn build(platform: &mut Platform, contents: &WidgetId) -> Self {
        let id = platform.next_id();

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createTransform"),
                jni_sig!((long)),
                &[id.into()],
            )?
            .v()?;

            env.call_method(
                activity,
                jni_str!("transformSetContents"),
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

    fn set_content_transform(
        &mut self,
        platform: &mut Platform,
        width: f32,
        height: f32,
        affine: Affine,
    ) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("transformSetTransform"),
                jni_sig!((
                    long, float, float, float, float, float, float, float
                )),
                &[
                    self.id.into(),
                    width.into(),
                    height.into(),
                    affine.offset_x.into(),
                    affine.offset_y.into(),
                    affine.rotation.into(),
                    affine.scale_x.into(),
                    affine.scale_y.into(),
                ],
            )?
            .v()
        });
    }
}
