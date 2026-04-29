use keyboard_types::Modifiers;
use ori::{Action, Message, Mut, Proxied, Proxy, Tracker, View, ViewId, ViewMarker};

use crate::{
    Context, Input, InputHandler, MatchKey, Platform, PressableEvent, WidgetView,
    widget::WidgetMut, widgets::PressableWidget,
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
    on_event: Box<dyn FnMut(&mut T, PressableEvent) -> Action>,
    on_press: Box<dyn FnMut(&mut T) -> Action>,
    on_hover: Box<dyn FnMut(&mut T, bool) -> Action>,
    on_focus: Box<dyn FnMut(&mut T, bool) -> Action>,
    on_move:  Box<dyn FnMut(&mut T, f32, f32) -> Action>,
    input:    Input<T>,
}

impl<V, T> Pressable<V, T> {
    /// Create new [`Pressable`].
    pub fn new(build: impl FnMut(&T, PressState) -> V + 'static) -> Self {
        Self {
            build:    Box::new(build),
            on_event: Box::new(|_, _| Action::new()),
            on_press: Box::new(|_| Action::new()),
            on_hover: Box::new(|_, _| Action::new()),
            on_focus: Box::new(|_, _| Action::new()),
            on_move:  Box::new(|_, _, _| Action::new()),
            input:    Input::new(),
        }
    }

    /// Set the callback for all events.
    pub fn on_event<A>(
        mut self,
        mut on_event: impl FnMut(&mut T, PressableEvent) -> A + 'static,
    ) -> Self
    where
        A: Into<Action>,
    {
        self.on_event = Box::new(move |data, event| on_event(data, event).into());
        self
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

    /// Set the callback for when the pointer is moved over the [`View`].
    pub fn on_move<A>(mut self, mut on_focus: impl FnMut(&mut T, f32, f32) -> A + 'static) -> Self
    where
        A: Into<Action>,
    {
        self.on_move = Box::new(move |data, x, y| on_focus(data, x, y).into());
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

impl<T, V> ViewMarker for Pressable<V, T> {}
impl<P, T, V> View<Context<P>, T> for Pressable<V, T>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = PressableWidget<P, V::Element>;
    type State = PressableState<P, T, V>;

    fn build(mut self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let press = PressState {
            pressed: false,
            hovered: false,
            focused: false,
        };

        let view = (self.build)(data, press);
        let (contents, state) = view.build(cx, data);

        let view_id = ViewId::next();
        cx.register(view_id);

        let on_event = {
            let proxy = cx.proxy();

            move |event| {
                proxy.message(Message::new(event, view_id));
            }
        };

        let mut widget = PressableWidget::new(cx, contents, on_event);

        let (filter, handler) = self.input.split();

        let proxy = cx.proxy();
        widget.set_on_key(cx, move |key, modifiers, pressed| {
            if let Some(message) = filter.filter_key(key, modifiers, pressed) {
                proxy.message(Message::new(message, view_id));
                true
            } else {
                false
            }
        });

        let state = PressableState {
            state,
            press,
            view_id,
            build: self.build,
            on_event: self.on_event,
            on_press: self.on_press,
            on_hover: self.on_hover,
            on_focus: self.on_focus,
            on_move: self.on_move,
            handler,
        };

        (widget, state)
    }

    fn rebuild(
        mut self,
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        {
            let view = (self.build)(data, state.press);
            let (mut parent, contents) = element.contents_mut();
            let widget = WidgetMut::new(&mut parent, contents);
            view.rebuild(widget, &mut state.state, cx, data);
        }

        let (filter, handler) = self.input.split();
        let proxy = cx.proxy();

        element.set_on_key(cx, {
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

        state.build = self.build;
        state.on_event = self.on_event;
        state.on_press = self.on_press;
        state.on_hover = self.on_hover;
        state.on_focus = self.on_focus;
        state.on_move = self.on_move;
        state.handler = handler;
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(message) = message.take(state.view_id) {
            return state.handler.handle(data, message);
        }

        if let Some(event) = message.take(state.view_id) {
            let mut action = Action::new();

            match event {
                PressableEvent::Pressed(_) => {
                    state.press.pressed = true;
                }

                PressableEvent::Released(_) | PressableEvent::Cancelled(_) => {
                    state.press.pressed = false;

                    if let PressableEvent::Released(_) = event {
                        action |= (state.on_press)(data);
                    }
                }

                PressableEvent::Moved(_) => {}

                PressableEvent::Hovered(hovered) => {
                    state.press.hovered = hovered;
                    action |= (state.on_hover)(data, hovered);
                }

                PressableEvent::Focused(focused) => {
                    state.press.focused = focused;
                    action |= (state.on_focus)(data, focused);
                }
            }

            action |= (state.on_event)(data, event);

            let (mut parent, contents) = element.contents_mut();
            let widget = WidgetMut::new(&mut parent, contents);

            let view = (state.build)(data, state.press);
            view.rebuild(widget, &mut state.state, cx, data);

            return action;
        }

        let (mut parent, contents) = element.contents_mut();
        let widget = WidgetMut::new(&mut parent, contents);

        V::message(
            widget,
            &mut state.state,
            cx,
            data,
            message,
        )
    }

    fn teardown(element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        let element = element.teardown(cx);
        V::teardown(element, state.state, cx);
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
    state:    V::State,
    press:    PressState,
    view_id:  ViewId,
    build:    Box<dyn FnMut(&T, PressState) -> V>,
    on_event: Box<dyn FnMut(&mut T, PressableEvent) -> Action>,
    on_press: Box<dyn FnMut(&mut T) -> Action>,
    on_hover: Box<dyn FnMut(&mut T, bool) -> Action>,
    on_focus: Box<dyn FnMut(&mut T, bool) -> Action>,
    on_move:  Box<dyn FnMut(&mut T, f32, f32) -> Action>,
    handler:  InputHandler<T>,
}
