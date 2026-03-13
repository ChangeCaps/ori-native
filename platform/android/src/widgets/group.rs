use jni::{jni_sig, jni_str};
use ori_native_core::{Color, NativeParent, NativeWidget, Overflow, Shadow, native::NativeGroup};

use crate::{Platform, platform::WidgetId};

pub struct Group {
    id: WidgetId,
}

impl NativeWidget<Platform> for Group {
    fn widget(&self) -> &WidgetId {
        &self.id
    }
}

impl NativeParent<Platform> for Group {
    fn replace_child(&mut self, platform: &mut Platform, index: usize, child: &WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupRemove"),
                jni_sig!((long, int)),
                &[self.id.into(), (index as i32).into()],
            )?
            .v()?;

            env.call_method(
                activity,
                jni_str!("groupInsert"),
                jni_sig!((long, int, long)),
                &[self.id.into(), (index as i32).into(), child.into()],
            )?
            .v()
        });
    }
}

impl NativeGroup<Platform> for Group {
    fn build(platform: &mut Platform) -> Self {
        let id = platform.next_id();

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("createGroup"),
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

    fn insert_child(&mut self, platform: &mut Platform, index: usize, child: &WidgetId) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupInsert"),
                jni_sig!((long, int, long)),
                &[self.id.into(), (index as i32).into(), child.into()],
            )?
            .v()
        });
    }

    fn remove_child(&mut self, platform: &mut Platform, index: usize) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupRemove"),
                jni_sig!((long, int)),
                &[self.id.into(), (index as i32).into()],
            )?
            .v()
        });
    }

    fn swap_children(&mut self, platform: &mut Platform, index_a: usize, index_b: usize) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSwap"),
                jni_sig!((long, int, int)),
                &[
                    self.id.into(),
                    (index_a as i32).into(),
                    (index_b as i32).into(),
                ],
            )?
            .v()
        });
    }

    fn set_child_layout(
        &mut self,
        platform: &mut Platform,
        index: usize,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSetChildLayout"),
                jni_sig!((long, int, float, float, float, float)),
                &[
                    self.id.into(),
                    (index as i32).into(),
                    x.into(),
                    y.into(),
                    width.into(),
                    height.into(),
                ],
            )?
            .v()
        });
    }

    fn set_background_color(&mut self, platform: &mut Platform, color: Color) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSetBackgroundColor"),
                jni_sig!((long, float, float, float, float)),
                &[
                    self.id.into(),
                    color.r.into(),
                    color.g.into(),
                    color.b.into(),
                    color.a.into(),
                ],
            )?
            .v()
        });
    }

    fn set_border_color(&mut self, platform: &mut Platform, color: Color) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSetBorderColor"),
                jni_sig!((long, float, float, float, float)),
                &[
                    self.id.into(),
                    color.r.into(),
                    color.g.into(),
                    color.b.into(),
                    color.a.into(),
                ],
            )?
            .v()
        });
    }

    fn set_border_width(&mut self, platform: &mut Platform, width: [f32; 4]) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSetBorderWidth"),
                jni_sig!((long, float, float, float, float)),
                &[
                    self.id.into(),
                    width[0].into(),
                    width[1].into(),
                    width[2].into(),
                    width[3].into(),
                ],
            )?
            .v()
        });
    }

    fn set_corner_radii(&mut self, platform: &mut Platform, radii: [f32; 4]) {
        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSetCornerRadii"),
                jni_sig!((long, float, float, float, float)),
                &[
                    self.id.into(),
                    radii[0].into(),
                    radii[1].into(),
                    radii[2].into(),
                    radii[3].into(),
                ],
            )?
            .v()
        });
    }

    fn set_overflow(&mut self, platform: &mut Platform, overflow: Overflow) {
        let visible = matches!(overflow, Overflow::Visible);

        let _ = platform.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("groupSetOverflow"),
                jni_sig!((long, boolean)),
                &[self.id.into(), visible.into()],
            )?
            .v()
        });
    }

    fn set_shadow(&mut self, _platform: &mut Platform, _shadow: Shadow) {}
}
