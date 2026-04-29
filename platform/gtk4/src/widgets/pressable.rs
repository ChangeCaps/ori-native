use std::{cell::Cell, rc::Rc};

use glib::object::{Cast, ObjectExt};
use gtk4::prelude::{AccessibleExt, FixedExt, GestureExt, WidgetExt};
use ori_native_core::{
    Key, Modifiers, NativeWidget, Pointer, PressableEvent, native::NativePressable,
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
        on_event: impl Fn(PressableEvent) + 'static,
    ) -> Self {
        let fixed = gtk4::Fixed::new();
        fixed.put(&contents, 0.0, 0.0);
        fixed.set_focusable(true);
        fixed.set_accessible_role(gtk4::AccessibleRole::Button);
        fixed.set_overflow(gtk4::Overflow::Visible);

        let on_event = Rc::new(on_event);

        let controller = gtk4::GestureClick::new();
        controller.connect_pressed({
            let on_event = on_event.clone();
            move |click, _, x, y| {
                click.set_state(gtk4::EventSequenceState::Claimed);

                let pointer = Pointer {
                    x: x as f32,
                    y: y as f32,
                };

                on_event(PressableEvent::Pressed(pointer));
            }
        });

        controller.connect_released({
            let on_event = on_event.clone();
            move |_, _, x, y| {
                let pointer = Pointer {
                    x: x as f32,
                    y: y as f32,
                };

                on_event(PressableEvent::Released(pointer));
            }
        });

        controller.connect_unpaired_release({
            let on_event = on_event.clone();
            move |_, x, y, _, _| {
                let pointer = Pointer {
                    x: x as f32,
                    y: y as f32,
                };

                on_event(PressableEvent::Cancelled(pointer))
            }
        });

        fixed.add_controller(controller);

        let hovered = Rc::new(Cell::new(false));

        let controller = gtk4::EventControllerMotion::new();
        controller.connect_motion({
            let fixed = fixed.downgrade();
            let on_event = on_event.clone();
            let hovered = hovered.clone();

            move |_, x, y| {
                if let Some(fixed) = fixed.upgrade()
                    && x > 0.0
                    && y > 0.0
                    && x < fixed.width() as f64
                    && y < fixed.height() as f64
                    && !hovered.get()
                {
                    on_event(PressableEvent::Hovered(true));
                    hovered.set(true);
                }

                let pointer = Pointer {
                    x: x as f32,
                    y: y as f32,
                };

                on_event(PressableEvent::Moved(pointer));
            }
        });

        controller.connect_leave({
            let on_event = on_event.clone();
            move |_| {
                on_event(PressableEvent::Hovered(false));
                hovered.set(false);
            }
        });

        fixed.add_controller(controller);

        let controller = gtk4::EventControllerFocus::new();
        controller.connect_enter({
            let on_event = on_event.clone();
            move |_| on_event(PressableEvent::Focused(true))
        });

        controller.connect_leave({
            let on_event = on_event.clone();
            move |_| on_event(PressableEvent::Focused(false))
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
