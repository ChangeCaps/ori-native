use std::time::Duration;

use keyboard_types::Modifiers;
use ori::{Action, Message, Mut, Proxied, Proxy, Tracker, View, ViewId, ViewMarker};

use crate::{
    AnimateRequest, Context, Input, InputHandler, LayoutRequest, Lifecycle, MatchKey, NativeWidget,
    NavigationBar, Platform, Pod, Sizing, StatusBar, WidgetView, native::NativeWindow,
};

/// [`View`] of a window.
pub fn window<T, V>(contents: V) -> Window<T, V> {
    Window::new(contents)
}

/// A window [`View`].
pub struct Window<T, V> {
    contents:   V,
    attributes: WindowAttributes<T>,
}

impl<T, V> Window<T, V> {
    /// Create new [`Window`].
    pub fn new(contents: V) -> Self {
        Window {
            contents,
            attributes: WindowAttributes::default(),
        }
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.attributes.title = title.into();
        self
    }

    /// Set the [`Sizing`].
    pub fn sizing(mut self, sizing: Sizing) -> Self {
        self.attributes.sizing = sizing;
        self
    }

    /// Set the style of the [`StatusBar`].
    pub fn status_bar(mut self, status_bar: StatusBar) -> Self {
        self.attributes.status_bar = status_bar;
        self
    }

    /// Set the style of the [`NavigationBar`].
    pub fn navigation_bar(mut self, navigation_bar: NavigationBar) -> Self {
        self.attributes.navigation_bar = navigation_bar;
        self
    }

    /// Add an callback for when a `key` is pressed.
    pub fn on_key<A>(
        mut self,
        key: impl MatchKey + 'static,
        mods: Modifiers,
        on_key: impl FnMut(&mut T) -> A + 'static,
    ) -> Self
    where
        A: Into<Action>,
    {
        self.attributes.input.add_key(key, mods, on_key);
        self
    }
}

#[derive(Debug)]
pub enum WindowMessage {
    AnimationFrame(Duration),
    CloseRequested,
    Resized,
}

impl<T, V> ViewMarker for Window<T, V> {}
impl<P, T, V> View<Context<P>, T> for Window<T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = ();
    type State = WindowState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let view_id = ViewId::next();

        let (contents, state) = cx.with_window(view_id, |cx| {
            self.contents.build(cx, data)
        });

        let window = P::Window::build(
            &mut cx.platform,
            contents.widget.widget(),
        );

        let state = WindowState::new(
            cx,
            data,
            window,
            view_id,
            self.attributes,
            contents,
            state,
        );

        ((), state)
    }

    fn rebuild(
        self,
        _element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        state.rebuild(cx, data, self.contents, self.attributes);
    }

    fn message(
        _element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        state.message(cx, data, message)
    }

    fn teardown(_element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        state.teardown(cx);
    }
}

/// Common attributes of a [`Window`].
pub struct WindowAttributes<T> {
    /// The title of the window.
    pub title: String,

    /// The sizing mode of the window.
    pub sizing: Sizing,

    /// The input handlers of the window.
    pub input: Input<T>,

    /// The style of the status bar.
    pub status_bar: StatusBar,

    /// The style of the navigation bar.
    pub navigation_bar: NavigationBar,
}

impl<T> Default for WindowAttributes<T> {
    fn default() -> Self {
        Self {
            title:  String::new(),
            sizing: Sizing::User,
            input:  Default::default(),

            status_bar:     Default::default(),
            navigation_bar: Default::default(),
        }
    }
}

/// Common state of a [`Window`].
pub struct WindowState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    /// The native window.
    pub window:  P::Window,
    /// The id of the view.
    pub view_id: ViewId,

    node:           taffy::NodeId,
    layout:         taffy::Layout,
    content_layout: taffy::Layout,

    status_bar:     StatusBar,
    navigation_bar: NavigationBar,

    title:   String,
    sizing:  Sizing,
    handler: InputHandler<T>,

    width:  u32,
    height: u32,

    animating: u32,

    contents: Pod<P, V::Widget>,
    state:    V::State,
}

impl<P, T, V> WindowState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    /// Create new [`WindowState`].
    pub fn new(
        cx: &mut Context<P>,
        data: &mut T,
        mut window: P::Window,
        view_id: ViewId,
        attributes: WindowAttributes<T>,
        contents: Pod<P, V::Widget>,
        state: V::State,
    ) -> Self {
        window.set_title(
            &mut cx.platform,
            attributes.title.clone(),
        );
        window.set_resizable(
            &mut cx.platform,
            matches!(attributes.sizing, Sizing::User),
        );

        let proxy = cx.proxy();

        window.set_on_resize(&mut cx.platform, {
            let proxy = proxy.cloned();

            move || {
                proxy.message(Message::new(
                    WindowMessage::Resized,
                    view_id,
                ));
            }
        });

        window.set_on_close_requested(&mut cx.platform, {
            let proxy = proxy.cloned();

            move || {
                proxy.message(Message::new(
                    WindowMessage::CloseRequested,
                    view_id,
                ));
            }
        });

        window.set_on_animation_frame(&mut cx.platform, {
            let proxy = proxy.cloned();

            move |delta| {
                proxy.message(Message::new(
                    WindowMessage::AnimationFrame(delta),
                    view_id,
                ));
            }
        });

        let (filter, handler) = attributes.input.split();

        window.set_on_key(&mut cx.platform, {
            let proxy = proxy.cloned();

            move |key, modifiers, pressed| {
                if let Some(message) = filter.filter_key(key, modifiers, pressed) {
                    proxy.message(Message::new(message, view_id));
                    true
                } else {
                    false
                }
            }
        });

        window.set_status_bar(&mut cx.platform, attributes.status_bar);

        window.set_navigation_bar(
            &mut cx.platform,
            attributes.navigation_bar,
        );

        cx.register(view_id);

        let node = cx.new_layout_node(Default::default(), &[contents.node]);
        let (width, height) = window.get_size(&mut cx.platform);

        let mut state = Self {
            window,
            view_id,
            node,
            layout: Default::default(),
            content_layout: Default::default(),
            title: attributes.title,
            sizing: attributes.sizing,
            handler,
            status_bar: attributes.status_bar,
            navigation_bar: attributes.navigation_bar,
            width,
            height,
            animating: 0,
            contents,
            state,
        };

        let action = state.layout(cx, data);
        cx.send_action(action);

        state
    }

    /// Rebuild `self`.
    pub fn rebuild(
        &mut self,
        cx: &mut Context<P>,
        data: &mut T,
        contents: V,
        attributes: WindowAttributes<T>,
    ) {
        cx.with_window(self.view_id, |cx| {
            contents.rebuild(
                (self.contents).as_mut(self.contents.node, &mut self.window, 0),
                &mut self.state,
                cx,
                data,
            );
        });

        let (filter, handler) = attributes.input.split();

        let proxy = cx.proxy();
        self.window.set_on_key(&mut cx.platform, {
            let view_id = self.view_id;

            move |key, modifiers, pressed| {
                if let Some(message) = filter.filter_key(key, modifiers, pressed) {
                    proxy.message(Message::new(message, view_id));
                    true
                } else {
                    false
                }
            }
        });

        self.handler = handler;

        if self.title != attributes.title {
            self.title = attributes.title.clone();
            self.window.set_title(
                &mut cx.platform,
                attributes.title.clone(),
            );
        }

        if self.sizing != attributes.sizing {
            self.sizing = attributes.sizing;
            self.window.set_resizable(
                &mut cx.platform,
                matches!(attributes.sizing, Sizing::User,),
            );
        }

        if self.status_bar != attributes.status_bar {
            self.status_bar = attributes.status_bar;
            (self.window).set_status_bar(&mut cx.platform, self.status_bar);
        }

        if self.navigation_bar != attributes.navigation_bar {
            self.navigation_bar = attributes.navigation_bar;
            (self.window).set_navigation_bar(&mut cx.platform, self.navigation_bar);
        }
    }

    /// Compute layout and potentially resize the window.
    pub fn layout(&mut self, cx: &mut Context<P>, data: &mut T) -> Action {
        let (width, height) = self.window.get_size(&mut cx.platform);

        self.width = width;
        self.height = height;

        if let Sizing::User = self.sizing {
            let style = taffy::Style {
                max_size: taffy::Size::from_lengths(0.0, 0.0),
                ..Default::default()
            };

            let size = taffy::Size {
                width:  taffy::AvailableSpace::MinContent,
                height: taffy::AvailableSpace::MinContent,
            };

            let _ = cx.set_layout_style(self.node, style);
            let _ = cx.compute_layout(self.node, size);

            if let Ok(layout) = cx.get_computed_layout(self.node).copied() {
                self.window.set_min_size(
                    &mut cx.platform,
                    layout.content_size.width as u32,
                    layout.content_size.height as u32,
                );
            }
        }

        let style = match self.sizing {
            Sizing::User => taffy::Style {
                size: taffy::Size::from_lengths(width as f32, height as f32),
                ..Default::default()
            },

            Sizing::Content => {
                let mut size = taffy::Size::auto();

                let (preferred_width, preferred_height) =
                    self.window.get_preferred_size(&mut cx.platform);

                if let Some(min_width) = preferred_width {
                    size.width = taffy::Dimension::length(min_width as f32);
                }

                if let Some(min_height) = preferred_height {
                    size.height = taffy::Dimension::length(min_height as f32);
                }

                taffy::Style {
                    size,
                    ..Default::default()
                }
            }
        };

        let size = match self.sizing {
            Sizing::User => taffy::Size {
                width:  taffy::AvailableSpace::Definite(width as f32),
                height: taffy::AvailableSpace::Definite(height as f32),
            },

            Sizing::Content => taffy::Size::max_content(),
        };

        let _ = cx.set_layout_style(self.node, style);
        let _ = cx.compute_layout(self.node, size);

        if let Ok(layout) = cx.get_computed_layout(self.node).copied()
            && self.layout != layout
        {
            self.layout = layout;

            if let Sizing::Content = self.sizing {
                self.window.set_size(
                    &mut cx.platform,
                    layout.size.width.round() as u32,
                    layout.size.height.round() as u32,
                );
            }
        }

        if let Ok(layout) = cx.get_computed_layout(self.contents.node).copied()
            && self.content_layout != layout
        {
            self.content_layout = layout;

            self.window.set_content_layout(
                &mut cx.platform,
                layout.location.x,
                layout.location.y,
                layout.size.width,
                layout.size.height,
            );
        }

        cx.with_window(self.view_id, |cx| {
            V::message(
                self.contents.as_mut(self.node, &mut self.window, 0),
                &mut self.state,
                cx,
                data,
                &mut Message::new(Lifecycle::Layout, None),
            )
        })
    }

    /// Handle a [`Message`].
    pub fn message(&mut self, cx: &mut Context<P>, data: &mut T, message: &mut Message) -> Action {
        if let Some(message) = message.take_targeted(self.view_id) {
            return self.handler.handle(data, message);
        }

        if let Some(message) = message.take_targeted(self.view_id) {
            return match message {
                LayoutRequest::Relayout => self.layout(cx, data),
            };
        }

        if let Some(message) = message.take_targeted(self.view_id) {
            return match message {
                AnimateRequest::Start => {
                    if self.animating == 0 {
                        self.window.start_animating(&mut cx.platform);
                    }

                    self.animating += 1;

                    Action::new()
                }

                AnimateRequest::Stop => {
                    self.animating -= 1;

                    if self.animating == 0 {
                        self.window.stop_animating(&mut cx.platform);
                    }

                    Action::new()
                }
            };
        }

        if let Some(message) = message.take_targeted(self.view_id) {
            return match message {
                WindowMessage::AnimationFrame(delta) => {
                    if self.animating == 0 {
                        return Action::new();
                    }

                    let mut message = Message::new(Lifecycle::Animate(delta), None);

                    cx.with_window(self.view_id, |cx| {
                        V::message(
                            self.contents.as_mut(self.node, &mut self.window, 0),
                            &mut self.state,
                            cx,
                            data,
                            &mut message,
                        )
                    })
                }

                WindowMessage::CloseRequested => {
                    cx.platform.quit();

                    Action::new()
                }

                WindowMessage::Resized => {
                    let (width, height) = self.window.get_size(&mut cx.platform);

                    if self.width != width || self.height != height {
                        self.layout(cx, data)
                    } else {
                        Action::new()
                    }
                }
            };
        }

        cx.with_window(self.view_id, |cx| {
            V::message(
                self.contents.as_mut(self.node, &mut self.window, 0),
                &mut self.state,
                cx,
                data,
                message,
            )
        })
    }

    /// Teardown the window state.
    pub fn teardown(self, cx: &mut Context<P>) {
        cx.with_window(self.view_id, |cx| {
            V::teardown(self.contents, self.state, cx);
        });

        self.window.teardown(&mut cx.platform);
        let _ = cx.remove_layout_node(self.node);
        cx.unregister(self.view_id);
    }
}
