use glib::object::Cast;
use gtk4::prelude::{FixedExt, WidgetExt};
use ori_native_core::{Affine, NativeWidget, native::NativeTransform};

use crate::Platform;

pub struct Transform {
    fixed: gtk4::Fixed,
}

impl NativeWidget<Platform> for Transform {
    fn widget_ref(&self) -> gtk4::Widget {
        self.fixed.clone().upcast()
    }
}

impl NativeTransform<Platform> for Transform {
    fn build(_platform: &mut Platform, contents: gtk4::Widget) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.set_overflow(gtk4::Overflow::Visible);
        fixed.put(&contents, 0.0, 0.0);

        Self { fixed }
    }

    fn teardown(self, _platform: &mut Platform) {}

    fn replace_contents(&mut self, _platform: &mut Platform, child: gtk4::Widget) {
        if let Some(child) = self.fixed.first_child() {
            self.fixed.remove(&child);
        }

        self.fixed.put(&child, 0.0, 0.0);
    }

    fn set_content_transform(
        &mut self,
        _platform: &mut Platform,
        width: f32,
        height: f32,
        affine: Affine,
    ) {
        if let Some(child) = self.fixed.first_child() {
            let transform = gsk4::Transform::new();
            let transform = transform.translate(&graphene::Point::new(
                affine.offset_x + width / 2.0,
                affine.offset_y + height / 2.0,
            ));
            let transform = transform.rotate(affine.rotation);
            let transform = transform.scale(affine.scale_x, affine.scale_y);
            let transform = transform.translate(&graphene::Point::new(
                -width / 2.0,
                -height / 2.0,
            ));

            self.fixed.set_child_transform(&child, Some(&transform));

            child.set_size_request(
                width.round() as i32,
                height.round() as i32,
            );
        }
    }
}
