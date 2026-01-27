use gtk4::prelude::{FixedExt, WidgetExt};
use ori_native_core::widgets::{HasGroup, NativeGroup};

use crate::Gtk4Platform;

pub struct Group {
    fixed:    gtk4::Fixed,
    children: Vec<gtk4::Widget>,
}

impl NativeGroup<Gtk4Platform> for Group {
    fn widget(&self) -> &gtk4::Widget {
        self.fixed.as_ref()
    }

    fn build(_platform: &mut Gtk4Platform) -> Self {
        Self {
            fixed:    gtk4::Fixed::new(),
            children: Vec::new(),
        }
    }

    fn teardown(self, _platform: &mut Gtk4Platform) {}

    fn insert_child(&mut self, index: usize, child: &gtk4::Widget) {
        self.fixed.put(child, 0.0, 0.0);
        self.children.insert(index, child.clone());
    }

    fn remove_child(&mut self, index: usize) {
        let child = self.children.remove(index);
        self.fixed.remove(&child);
    }

    fn swap_children(&mut self, index_a: usize, index_b: usize) {
        self.children.swap(index_a, index_b);
    }

    fn set_size(&mut self, width: f32, height: f32) {
        self.fixed.set_size_request(width as i32, height as i32);
    }

    fn set_child_position(&mut self, index: usize, x: f32, y: f32) {
        self.fixed.move_(
            &self.children[index],
            x as f64,
            y as f64,
        );
    }
}

impl HasGroup for Gtk4Platform {
    type Group = Group;
}
