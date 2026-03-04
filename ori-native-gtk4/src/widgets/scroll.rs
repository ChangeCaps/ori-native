use ori_native_core::{
    Direction, NativeParent, NativeWidget,
    native::{HasScroll, NativeScroll},
};

use crate::{Platform, widgets::group::GroupWidget};

impl HasScroll for Platform {
    type Scroll = Scroll;
}

pub struct Scroll {
    scroll: gtk4::ScrolledWindow,
    group:  GroupWidget,
}

impl NativeWidget<Platform> for Scroll {
    fn widget(&self) -> &gtk4::Widget {
        self.scroll.as_ref()
    }
}

impl NativeParent<Platform> for Scroll {
    fn replace_child(&mut self, _platform: &mut Platform, index: usize, child: &gtk4::Widget) {
        debug_assert_eq!(index, 0);

        self.group.replace_child(0, child);
    }
}

impl NativeScroll<Platform> for Scroll {
    fn build(_platform: &mut Platform, contents: &gtk4::Widget) -> Self {
        let scroll = gtk4::ScrolledWindow::new();
        let group = GroupWidget::new();
        group.insert_child(0, contents);
        scroll.set_child(Some(&group));

        Self { scroll, group }
    }

    fn teardown(self, _platform: &mut Platform) {}

    fn set_content_size(&mut self, width: f32, height: f32) {
        self.group.set_size(
            width.round() as i32,
            height.round() as i32,
        );
    }

    fn set_content_layout(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.group.set_child_layout(
            0,
            x.round() as i32,
            y.round() as i32,
            width.round() as i32,
            height.round() as i32,
        );
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
