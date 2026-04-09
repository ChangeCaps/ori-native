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
    fn build(_platform: &mut Platform, contents: &gtk4::Widget) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(contents, 0.0, 0.0);

        Self {
            contents: contents.clone(),
            fixed,
        }
    }

    fn teardown(self, _platform: &mut Platform) {}

    fn set_content_size(&mut self, _platform: &mut Platform, width: f32, height: f32) {
        self.contents.set_size_request(
            width.round() as i32,
            height.round() as i32,
        );
    }

    fn measure(&mut self, _platform: &mut Platform) -> (f32, f32) {
        let origin = graphene::Point::zero();

        if let Some(root) = self.fixed.root()
            && let Some(point) = self.fixed.compute_point(&root, &origin)
        {
            (point.x(), point.y())
        } else {
            (0.0, 0.0)
        }
    }
}
