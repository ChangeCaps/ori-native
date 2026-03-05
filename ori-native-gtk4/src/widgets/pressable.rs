use std::rc::Rc;

use gtk4::prelude::{AccessibleExt, FixedExt, WidgetExt};
use ori_native_core::{
    Key, Modifiers, NativeParent, NativeWidget,
    native::{HasPressable, NativePressable, Press},
};

use crate::{Platform, key};

impl HasPressable for Platform {
    type Pressable = Pressable;
}

pub struct Pressable {
    fixed: gtk4::Fixed,
    press: Option<gtk4::GestureClick>,
    hover: Option<gtk4::EventControllerMotion>,
    focus: Option<gtk4::EventControllerFocus>,
    key:   Option<gtk4::EventControllerKey>,
}

impl NativeWidget<Platform> for Pressable {
    fn widget(&self) -> &gtk4::Widget {
        self.fixed.as_ref()
    }
}

impl NativeParent<Platform> for Pressable {
    fn replace_child(&mut self, _platform: &mut Platform, index: usize, child: &gtk4::Widget) {
        assert_eq!(index, 0);

        if let Some(child) = self.fixed.first_child() {
            self.fixed.remove(&child);
        }

        self.fixed.put(child, 0.0, 0.0);
    }
}

impl NativePressable<Platform> for Pressable {
    fn build(_plaform: &mut Platform, contents: &gtk4::Widget) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(contents, 0.0, 0.0);
        fixed.set_focusable(true);
        fixed.set_accessible_role(gtk4::AccessibleRole::Button);
        fixed.set_overflow(gtk4::Overflow::Visible);

        Self {
            fixed,
            press: None,
            hover: None,
            focus: None,
            key: None,
        }
    }

    fn teardown(self, _plaform: &mut Platform) {}

    fn set_content_size(&mut self, _plaform: &mut Platform, width: f32, height: f32) {
        if let Some(child) = self.fixed.first_child() {
            child.set_size_request(
                width.round() as i32,
                height.round() as i32,
            );
        }
    }

    fn set_on_press(&mut self, on_press: impl Fn(Press) + 'static) {
        if let Some(press) = self.press.take() {
            self.fixed.remove_controller(&press);
        }

        let on_press = Rc::new(on_press);

        let controller = gtk4::GestureClick::new();
        controller.connect_pressed({
            let on_press = on_press.clone();
            move |_, _, _, _| on_press(Press::Pressed)
        });

        controller.connect_released({
            let on_press = on_press.clone();
            move |_, _, _, _| on_press(Press::Released)
        });

        controller.connect_unpaired_release({
            let on_press = on_press.clone();
            move |_, _, _, _, _| on_press(Press::Cancelled)
        });

        self.press = Some(controller.clone());
        self.fixed.add_controller(controller);
    }

    fn set_on_hover(&mut self, on_hover: impl Fn(bool) + 'static) {
        if let Some(hover) = self.hover.take() {
            self.fixed.remove_controller(&hover);
        }

        let on_hover = Rc::new(on_hover);

        let controller = gtk4::EventControllerMotion::new();
        controller.connect_enter({
            let on_hover = on_hover.clone();
            move |_, _, _| on_hover(true)
        });

        controller.connect_leave({
            let on_hover = on_hover.clone();
            move |_| on_hover(false)
        });

        self.hover = Some(controller.clone());
        self.fixed.add_controller(controller);
    }

    fn set_on_focus(&mut self, on_focus: impl Fn(bool) + 'static) {
        if let Some(focus) = self.focus.take() {
            self.fixed.remove_controller(&focus);
        }

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

        self.focus = Some(controller.clone());
        self.fixed.add_controller(controller);
    }

    fn set_on_key(&mut self, on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static) {
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
