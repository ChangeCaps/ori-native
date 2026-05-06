use glib::object::Cast;
use gtk4::prelude::{FixedExt, WidgetExt};
use ori_native_core::native::NativeMeasure;

use crate::Platform;

pub struct Measure {
    contents: gtk4::Widget,
    fixed:    gtk4::Fixed,
}

impl NativeMeasure<Platform> for Measure {
    fn build(
        platform: &mut Platform,
        contents: gtk4::Widget,
        on_position_changed: impl Fn(f32, f32) + 'static,
    ) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(&contents, 0.0, 0.0);

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

    fn widget_ref(&self) -> gtk4::Widget {
        self.fixed.clone().upcast()
    }

    fn replace_contents(&mut self, _platform: &mut Platform, child: gtk4::Widget) {
        self.fixed.remove(&self.contents);
        self.fixed.put(&child, 0.0, 0.0);
        self.contents = child.clone();
    }

    fn set_content_size(&mut self, _platform: &mut Platform, width: f32, height: f32) {
        self.contents.set_size_request(
            width.round() as i32,
            height.round() as i32,
        );
    }
}
