use std::{cell::Cell, rc::Rc};

use glib::object::{Cast, ObjectExt};
use gtk4::prelude::{AccessibleExt, FixedExt, GestureExt, WidgetExt};
use ori_native_core::{
    Key, Modifiers, NativeWidget,
    native::{NativePressable, Press},
};

use crate::{Platform, key};

pub struct Pressable {
    fixed: gtk4::Fixed,
    key:   Option<gtk4::EventControllerKey>,
}

impl NativeWidget<Platform> for Pressable {
    fn widget_ref(&self) -> gtk4::Widget {
        self.fixed.clone().upcast()
    }
}

impl NativePressable<Platform> for Pressable {
    fn build(
        _platform: &mut Platform,
        contents: gtk4::Widget,
        on_press: impl Fn(Press) + 'static,
        on_hover: impl Fn(bool) + 'static,
        on_focus: impl Fn(bool) + 'static,
    ) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(&contents, 0.0, 0.0);
        fixed.set_focusable(true);
        fixed.set_accessible_role(gtk4::AccessibleRole::Button);
        fixed.set_overflow(gtk4::Overflow::Visible);

        let on_press = Rc::new(on_press);

        let controller = gtk4::GestureClick::new();
        controller.connect_pressed({
            let on_press = on_press.clone();
            move |controller, _, _, _| {
                controller.set_state(gtk4::EventSequenceState::Claimed);
                on_press(Press::Pressed)
            }
        });

        controller.connect_released({
            let on_press = on_press.clone();
            move |_, _, _, _| on_press(Press::Released)
        });

        controller.connect_unpaired_release({
            let on_press = on_press.clone();
            move |_, _, _, _, _| on_press(Press::Cancelled)
        });

        fixed.add_controller(controller);

        let on_hover = Rc::new(on_hover);
        let hovered = Rc::new(Cell::new(false));

        let controller = gtk4::EventControllerMotion::new();
        controller.connect_motion({
            let fixed = fixed.downgrade();
            let on_hover = on_hover.clone();
            let hovered = hovered.clone();

            move |_, x, y| {
                if let Some(fixed) = fixed.upgrade()
                    && x > 0.0
                    && y > 0.0
                    && x < fixed.width() as f64
                    && y < fixed.height() as f64
                    && !hovered.get()
                {
                    on_hover(true);
                    hovered.set(true);
                }
            }
        });

        controller.connect_leave({
            let on_hover = on_hover.clone();
            move |_| {
                on_hover(false);
                hovered.set(false);
            }
        });

        fixed.add_controller(controller);

        let on_focus = Rc::new(on_focus);
        let controller = gtk4::EventControllerFocus::new();
        controller.connect_enter({
            let on_focus = on_focus.clone();
            move |_| on_focus(true)
        });

        controller.connect_leave({
            let on_focus = on_focus.clone();
            move |_| on_focus(false)
        });

        fixed.add_controller(controller);

        Self { fixed, key: None }
    }

    fn teardown(self, _platform: &mut Platform) {}

    fn replace_contents(&mut self, _platform: &mut Platform, child: gtk4::Widget) {
        if let Some(child) = self.fixed.first_child() {
            self.fixed.remove(&child);
        }

        self.fixed.put(&child, 0.0, 0.0);
    }

    fn set_content_size(&mut self, _platform: &mut Platform, width: f32, height: f32) {
        if let Some(child) = self.fixed.first_child() {
            child.set_size_request(
                width.round() as i32,
                height.round() as i32,
            );
        }
    }

    fn set_on_key(
        &mut self,
        _platform: &mut Platform,
        on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    ) {
        if let Some(key) = self.key.take() {
            self.fixed.remove_controller(&key);
        }

        let on_key = Rc::new(on_key);

        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed({
            let on_key = on_key.clone();

            move |_, key, _code, modifiers| {
                let key = key::convert_key(key);
                let modifiers = key::convert_modifiers(modifiers);

                if on_key(key, modifiers, true) {
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            }
        });

        controller.connect_key_released({
            let on_key = on_key.clone();

            move |_, key, _code, modifiers| {
                let key = key::convert_key(key);
                let modifiers = key::convert_modifiers(modifiers);
                on_key(key, modifiers, false);
            }
        });

        self.key = Some(controller.clone());
        self.fixed.add_controller(controller);
    }
}
