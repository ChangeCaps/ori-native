use keyboard_types::Modifiers;
use ori::{Action, Message, Mut, Proxied, Proxy, Tracker, View, ViewId, ViewMarker};

use crate::{
    Context, Input, InputHandler, Lifecycle, NativeWidget, Platform, Pod, WidgetView,
    input::MatchKey,
    native::{NativePressable, Press},
};

/// [`View`] that reacts to presses and focus.
pub fn pressable<V, T>(build: impl FnMut(&T, PressState) -> V + 'static) -> Pressable<V, T> {
    Pressable::new(build)
}

/// State of a [`Pressable`] [`View`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PressState {
    /// The view is pressed.
    pub pressed: bool,

    /// The view is hovered.
    pub hovered: bool,

    /// The view is focused.
    pub focused: bool,
}

/// [`View`] that reacts to presses and focus.
#[allow(clippy::type_complexity)]
pub struct Pressable<V, T> {
    build:    Box<dyn FnMut(&T, PressState) -> V>,
    on_press: Box<dyn FnMut(&mut T) -> Action>,
    on_hover: Box<dyn FnMut(&mut T, bool) -> Action>,
    on_focus: Box<dyn FnMut(&mut T, bool) -> Action>,
    input:    Input<T>,
}

impl<V, T> Pressable<V, T> {
    /// Create new [`Pressable`].
    pub fn new(build: impl FnMut(&T, PressState) -> V + 'static) -> Self {
        Self {
            build:    Box::new(build),
            on_press: Box::new(|_| Action::new()),
            on_hover: Box::new(|_, _| Action::new()),
            on_focus: Box::new(|_, _| Action::new()),
            input:    Input::new(),
        }
    }

    /// Set the callback for when the [`View`] is pressed.
    pub fn on_press<A>(mut self, mut on_press: impl FnMut(&mut T) -> A + 'static) -> Self
    where
        A: Into<Action>,
    {
        self.on_press = Box::new(move |data| on_press(data).into());
        self
    }

    /// Set the callback for when the [`View`] is hovered.
    pub fn on_hover<A>(mut self, mut on_hover: impl FnMut(&mut T, bool) -> A + 'static) -> Self
    where
        A: Into<Action>,
    {
        self.on_hover = Box::new(move |data, hovered| on_hover(data, hovered).into());
        self
    }

    /// Set the callback for when the [`View`] is focused.
    pub fn on_focus<A>(mut self, mut on_focus: impl FnMut(&mut T, bool) -> A + 'static) -> Self
    where
        A: Into<Action>,
    {
        self.on_focus = Box::new(move |data, focused| on_focus(data, focused).into());
        self
    }

    /// Set a callback for when `key` is pressed.
    pub fn on_key<A>(
        mut self,
        key: impl MatchKey + 'static,
        mods: Modifiers,
        on_key: impl FnMut(&mut T) -> A + 'static,
    ) -> Self
    where
        A: Into<Action>,
    {
        self.input.add_key(key, mods, on_key);
        self
    }
}

enum PressableMessage {
    Pressed(Press),
    Hovered(bool),
    Focused(bool),
}

impl<T, V> ViewMarker for Pressable<V, T> {}
impl<P, T, V> View<Context<P>, T> for Pressable<V, T>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Pressable>;
    type State = PressableState<P, T, V>;

    fn build(mut self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let press = PressState {
            pressed: false,
            hovered: false,
            focused: false,
        };

        let view = (self.build)(data, press);
        let (contents, state) = view.build(cx, data);

        let mut widget = P::Pressable::build(
            &mut cx.platform,
            contents.widget.widget(),
        );

        let view_id = ViewId::next();
        cx.register(view_id);

        let proxy = cx.proxy();

        widget.set_on_press(&mut cx.platform, {
            let proxy = proxy.cloned();

            move |pressed| {
                proxy.message(Message::new(
                    PressableMessage::Pressed(pressed),
                    view_id,
                ));
            }
        });

        widget.set_on_hover(&mut cx.platform, {
            let proxy = proxy.cloned();

            move |hovered| {
                proxy.message(Message::new(
                    PressableMessage::Hovered(hovered),
                    view_id,
                ));
            }
        });

        widget.set_on_focus(&mut cx.platform, {
            let proxy = proxy.cloned();

            move |focused| {
                proxy.message(Message::new(
                    PressableMessage::Focused(focused),
                    view_id,
                ));
            }
        });

        let (filter, handler) = self.input.split();

        widget.set_on_key(&mut cx.platform, {
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

        let pod = Pod::new(contents.node, widget);

        let state = PressableState {
            widget: contents.widget,
            state,
            press,
            view_id,
            layout: Default::default(),
            build: self.build,
            on_press: self.on_press,
            on_hover: self.on_hover,
            on_focus: self.on_focus,
            handler,
        };

        (pod, state)
    }

    fn rebuild(
        mut self,
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        let view = (self.build)(data, state.press);

        view.rebuild(
            element.map_widget(&mut state.widget),
            &mut state.state,
            cx,
            data,
        );
        state.build = self.build;
        state.on_press = self.on_press;

        let (filter, handler) = self.input.split();
        let proxy = cx.proxy();

        element.widget.set_on_key(&mut cx.platform, {
            let view_id = state.view_id;

            move |key, modifiers, pressed| {
                if let Some(message) = filter.filter_key(key, modifiers, pressed) {
                    proxy.message(Message::new(message, view_id));
                    true
                } else {
                    false
                }
            }
        });

        state.handler = handler;
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get()
            && let Ok(layout) = cx.get_computed_layout(*element.node).cloned()
            && state.layout != layout
        {
            state.layout = layout;
            element.widget.set_content_size(
                &mut cx.platform,
                layout.size.width,
                layout.size.height,
            );
        }

        if let Some(message) = message.take_targeted(state.view_id) {
            return state.handler.handle(data, message);
        }

        if let Some(message) = message.take_targeted(state.view_id) {
            let mut action = Action::new();

            match message {
                PressableMessage::Pressed(pressed) => {
                    state.press.pressed = matches!(pressed, Press::Pressed);

                    if let Press::Released = pressed {
                        action |= (state.on_press)(data);
                    }
                }

                PressableMessage::Hovered(hovered) => {
                    state.press.hovered = hovered;
                    action |= (state.on_hover)(data, hovered);
                }

                PressableMessage::Focused(focused) => {
                    state.press.focused = focused;
                    action |= (state.on_focus)(data, focused);
                }
            }

            let view = (state.build)(data, state.press);
            view.rebuild(
                element.map_widget(&mut state.widget),
                &mut state.state,
                cx,
                data,
            );

            action
        } else {
            V::message(
                element.map_widget(&mut state.widget),
                &mut state.state,
                cx,
                data,
                message,
            )
        }
    }

    fn teardown(element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        let pod = Pod::new(element.node, state.widget);

        V::teardown(pod, state.state, cx);
        element.widget.teardown(&mut cx.platform);
        cx.unregister(state.view_id);
    }
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub struct PressableState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    widget:   V::Widget,
    state:    V::State,
    press:    PressState,
    view_id:  ViewId,
    layout:   taffy::Layout,
    build:    Box<dyn FnMut(&T, PressState) -> V>,
    on_press: Box<dyn FnMut(&mut T) -> Action>,
    on_hover: Box<dyn FnMut(&mut T, bool) -> Action>,
    on_focus: Box<dyn FnMut(&mut T, bool) -> Action>,
    handler:  InputHandler<T>,
}
