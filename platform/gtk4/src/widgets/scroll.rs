use gtk4::prelude::{FixedExt, WidgetExt};
use ori_native_core::{Direction, NativeParent, NativeWidget, native::NativeScroll};

use crate::Platform;

pub struct Scroll {
    scroll: gtk4::ScrolledWindow,
    fixed:  gtk4::Fixed,
}

impl NativeWidget<Platform> for Scroll {
    fn widget(&self) -> &gtk4::Widget {
        self.scroll.as_ref()
    }
}

impl NativeParent<Platform> for Scroll {
    fn replace_child(&mut self, _platform: &mut Platform, index: usize, child: &gtk4::Widget) {
        debug_assert_eq!(index, 0);

        if let Some(child) = self.fixed.first_child() {
            self.fixed.remove(&child);
        }

        self.fixed.put(child, 0.0, 0.0);
    }
}

impl NativeScroll<Platform> for Scroll {
    fn build(_platform: &mut Platform, contents: &gtk4::Widget) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(contents, 0.0, 0.0);
        fixed.set_overflow(gtk4::Overflow::Visible);

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&fixed));

        Self { scroll, fixed }
    }

    fn teardown(self, _platform: &mut Platform) {}

    fn set_content_size(&mut self, width: f32, height: f32) {
        self.fixed.set_size_request(
            width.round() as i32,
            height.round() as i32,
        );
    }

    fn set_content_layout(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(child) = self.fixed.first_child() {
            self.fixed.move_(&child, x as f64, y as f64);

            child.set_size_request(
                width.round() as i32,
                height.round() as i32,
            );
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        self.scroll.set_hscrollbar_policy(match direction {
            Direction::Horizontal => gtk4::PolicyType::Automatic,
            Direction::Vertical => gtk4::PolicyType::Never,
        });

        self.scroll.set_vscrollbar_policy(match direction {
            Direction::Horizontal => gtk4::PolicyType::Never,
            Direction::Vertical => gtk4::PolicyType::Automatic,
        });
    }
}
