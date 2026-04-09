use glib::object::Cast;
use gtk4::prelude::{FixedExt, WidgetExt};
use ori_native_core::{NativeParent, NativeWidget, native::NativeMeasure};

use crate::Platform;

pub struct Measure {
    contents: gtk4::Widget,
    fixed:    gtk4::Fixed,
}

impl NativeParent<Platform> for Measure {
    fn replace_child(&mut self, _platform: &mut Platform, _index: usize, child: &gtk4::Widget) {
        self.fixed.remove(&self.contents);
        self.fixed.put(child, 0.0, 0.0);
        self.contents = child.clone();
    }
}

impl NativeWidget<Platform> for Measure {
    fn widget_ref(&self) -> &gtk4::Widget {
        self.fixed.as_ref()
    }
}

impl NativeMeasure<Platform> for Measure {
    fn build(
        platform: &mut Platform,
        contents: &gtk4::Widget,
        on_position_changed: impl Fn(f32, f32) + 'static,
    ) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(contents, 0.0, 0.0);

        let on_draw = {
            let fixed = fixed.clone();
            let mut x = 0.0;
            let mut y = 0.0;

            move || {
                let origin = graphene::Point::zero();

                if let Some(root) = fixed.root()
                    && let Some(point) = fixed.compute_point(&root, &origin)
                    && (x != point.x() || y != point.y())
                {
                    x = point.x();
                    y = point.y();

                    on_position_changed(x, y);
                }
            }
        };

        platform.on_snapshot.borrow_mut().insert(
            fixed.clone().upcast(),
            Box::new(on_draw),
        );

        Self {
            contents: contents.clone(),
            fixed,
        }
    }

    fn teardown(self, platform: &mut Platform) {
        let widget: &gtk4::Widget = self.fixed.as_ref();
        platform.on_snapshot.borrow_mut().remove(widget);
    }

    fn set_content_size(&mut self, _platform: &mut Platform, width: f32, height: f32) {
        self.contents.set_size_request(
            width.round() as i32,
            height.round() as i32,
        );
    }
}
