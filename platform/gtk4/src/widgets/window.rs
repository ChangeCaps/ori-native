use std::{rc::Rc, time::Duration};

use glib::{object::Cast, subclass::types::ObjectSubclassIsExt};
use gtk4::prelude::{FixedExt, GtkWindowExt, WidgetExt};
use ori_native_core::{Key, Modifiers, NativeParent, native::NativeWindow};

use crate::{Platform, key};

impl NativeParent<Platform> for Window {
    fn replace_child(&mut self, _platform: &mut Platform, index: usize, child: &gtk4::Widget) {
        debug_assert_eq!(index, 0);

        if let Some(child) = self.imp().fixed.first_child() {
            self.imp().fixed.remove(&child);
        }

        self.imp().fixed.put(child, 0.0, 0.0);
    }
}

impl NativeWindow<Platform> for Window {
    fn build(platform: &mut Platform, contents: &gtk4::Widget) -> Self {
        let window = Self::new(&platform.application);
        window.imp().fixed.put(contents, 0.0, 0.0);
        window.show();

        window
    }

    fn teardown(self, _platform: &mut Platform) {
        self.destroy();
    }

    fn get_size(&self, _platform: &mut Platform) -> (u32, u32) {
        (
            self.width() as u32,
            self.height() as u32,
        )
    }

    fn get_preferred_size(&self, _platform: &mut Platform) -> (Option<u32>, Option<u32>) {
        #[allow(unused_mut)]
        let mut min_width = None;
        #[allow(unused_mut)]
        let mut min_height = None;

        #[cfg(feature = "layer-shell")]
        {
            use gtk4_layer_shell::{Edge, LayerShell};

            if let Some(monitor) = self.monitor() {
                use gdk4::prelude::MonitorExt;

                let geometry = monitor.geometry();

                if self.is_anchor(Edge::Left) && self.is_anchor(Edge::Right) {
                    min_width = Some(geometry.width() as u32);
                }

                if self.is_anchor(Edge::Top) && self.is_anchor(Edge::Bottom) {
                    min_height = Some(geometry.height() as u32);
                }
            }
        }

        (min_width, min_height)
    }

    fn set_on_animation_frame(
        &mut self,
        _platform: &mut Platform,
        on_frame: impl Fn(Duration) + 'static,
    ) {
        if let Some(frame_clock) = self.frame_clock() {
            let previous = self.imp().previous_frame.clone();

            frame_clock.connect_before_paint(move |frame_clock| {
                let frame_time = frame_clock.frame_time();

                if let Some(previous) = previous.replace(Some(frame_time)) {
                    let delta = frame_time - previous;

                    if delta > 100 {
                        on_frame(Duration::from_micros(delta as u64));
                    }
                }
            });
        }
    }

    fn set_on_close_requested(
        &mut self,
        _platform: &mut Platform,
        on_close_requested: impl Fn() + 'static,
    ) {
        self.connect_close_request(move |_| {
            on_close_requested();
            gtk4::glib::Propagation::Stop
        });
    }

    fn set_on_resize(&mut self, _platform: &mut Platform, on_resize: impl Fn() + 'static) {
        self.set_on_size_allocate(on_resize);
    }

    fn set_on_key(
        &mut self,
        _platform: &mut Platform,
        on_key: impl Fn(Key, Modifiers, bool) -> bool + 'static,
    ) {
        for controller in self.observe_controllers().into_iter() {
            if let Ok(controller) = controller
                && let Ok(controller) = controller.dynamic_cast::<gtk4::EventControllerKey>()
            {
                self.remove_controller(&controller);
            }
        }

        let controller = gtk4::EventControllerKey::new();
        let on_key = Rc::new(on_key);

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

        self.add_controller(controller);
    }

    fn set_title(&mut self, _platform: &mut Platform, title: String) {
        gtk4::ApplicationWindow::set_title(self.as_ref(), Some(&title));
    }

    fn set_content_layout(
        &mut self,
        _platform: &mut Platform,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        if let Some(child) = self.imp().fixed.first_child() {
            self.imp().fixed.move_(&child, x as f64, y as f64);

            child.set_size_request(
                width.round() as i32,
                height.round() as i32,
            );
        }
    }

    fn set_min_size(&mut self, _platform: &mut Platform, width: u32, height: u32) {
        #[cfg(feature = "layer-shell")]
        {
            use gtk4_layer_shell::LayerShell;

            if self.is_layer_window() {
                return;
            }
        }

        self.set_size_request(width as i32, height as i32);
    }

    fn set_size(&mut self, _platform: &mut Platform, width: u32, height: u32) {
        self.set_default_size(
            width.max(1) as i32,
            height.max(1) as i32,
        );
    }

    fn set_resizable(&mut self, _platform: &mut Platform, resizable: bool) {
        gtk4::Window::set_resizable(self.as_ref(), resizable);
    }

    fn start_animating(&mut self, _platform: &mut Platform) {
        if let Some(frame_clock) = self.frame_clock() {
            frame_clock.begin_updating();
            self.imp().previous_frame.set(None);
        }
    }

    fn stop_animating(&mut self, _platform: &mut Platform) {
        if let Some(frame_clock) = self.frame_clock() {
            frame_clock.end_updating();
        }
    }
}

gtk4::glib::wrapper! {
    pub struct Window(
        ObjectSubclass<imp::ApplicationWindow>)
        @extends
            gtk4::ApplicationWindow,
            gtk4::Window,
            gtk4::Widget,
        @implements
            gtk4::Buildable,
            gtk4::Accessible,
            gtk4::ConstraintTarget,
            gtk4::Root,
            gtk4::Native,
            gtk4::ShortcutManager,
            gtk4::gio::ActionGroup,
            gtk4::gio::ActionMap;
}

impl Window {
    pub fn new(application: &gtk4::Application) -> Self {
        let window: Window = gtk4::glib::Object::builder().build();
        window.set_application(Some(application));
        window.set_child(Some(&window.imp().fixed));
        window
    }

    pub fn set_on_size_allocate(&self, on_size_allocate: impl Fn() + 'static) {
        let _ = self
            .imp()
            .on_size_allocate
            .replace(Box::new(on_size_allocate));
    }
}

mod imp {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::subclass::{
        prelude::ApplicationWindowImpl,
        widget::{WidgetImpl, WidgetImplExt},
        window::WindowImpl,
    };

    pub struct ApplicationWindow {
        pub fixed:            gtk4::Fixed,
        pub on_size_allocate: RefCell<Box<dyn Fn()>>,
        pub previous_frame:   Rc<Cell<Option<i64>>>,
    }

    impl Default for ApplicationWindow {
        fn default() -> Self {
            Self {
                fixed:            gtk4::Fixed::new(),
                on_size_allocate: RefCell::new(Box::new(|| {})),
                previous_frame:   Rc::new(Cell::new(None)),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "OriWindow";
        type Type = super::Window;
        type ParentType = gtk4::ApplicationWindow;
    }

    impl ObjectImpl for ApplicationWindow {}

    impl WidgetImpl for ApplicationWindow {
        fn size_allocate(&self, _width: i32, _height: i32, baseline: i32) {
            self.parent_size_allocate(i32::MAX, i32::MAX, baseline);
            let on_size_allocate = self.on_size_allocate.borrow();
            on_size_allocate();
        }

        fn measure(&self, _orientation: gtk4::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            (0, 0, -1, -1)
        }
    }

    impl WindowImpl for ApplicationWindow {}

    impl ApplicationWindowImpl for ApplicationWindow {}
}
