use ori::{Action, Message, Mut, Proxied, Proxy, Tracker, View, ViewId, ViewMarker};

use crate::{
    Allocation, Context, Direction, FlexStyle, Layout, LayoutStyle, Lifecycle, NativeWidget,
    Overflow, Platform, Pod, Size, WidgetView, native::NativeScroll,
};

/// [`View`] of a horizontal scroll area.
pub fn hscroll<T, V>(contents: V) -> Scroll<T, V> {
    Scroll::new(contents, Direction::Row)
}

/// [`View`] of a vertical scroll area.
pub fn vscroll<T, V>(contents: V) -> Scroll<T, V> {
    Scroll::new(contents, Direction::Column)
}

/// [`View`] of a scroll area.
#[allow(clippy::type_complexity)]
pub struct Scroll<T, V> {
    contents:  V,
    direction: Direction,
    layout:    LayoutStyle,
    on_scroll: Box<dyn FnMut(&mut T, f32) -> Action>,
}

impl<T, V> Scroll<T, V> {
    /// Create new [`Scroll`].
    pub fn new(contents: V, direction: Direction) -> Self {
        Self {
            contents,
            direction,
            layout: LayoutStyle::default(),
            on_scroll: Box::new(|_, _| Action::new()),
        }
    }

    /// Set callback when the view is scrolled.
    pub fn on_scroll<A>(mut self, mut on_scroll: impl FnMut(&mut T, f32) -> A + 'static) -> Self
    where
        A: Into<Action>,
    {
        self.on_scroll = Box::new(move |data, offset| on_scroll(data, offset).into());
        self
    }
}

impl<T, V> Layout for Scroll<T, V> {
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle {
        &mut self.layout
    }
}

struct ScrollMessage(f32, f32);

impl<T, V> ViewMarker for Scroll<T, V> {}
impl<P, T, V> View<Context<P>, T> for Scroll<T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Scroll>;
    type State = (V::Element, ScrollState<P, T, V>);

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents, state) = self.contents.build(cx, data);
        let node = cx.layout.add_node(&[contents.node]);
        cx.layout.set_layout(node, self.layout);

        let overflow = match self.direction {
            Direction::Row => Size {
                width:  Overflow::Hidden,
                height: Overflow::Visible,
            },

            Direction::Column => Size {
                width:  Overflow::Visible,
                height: Overflow::Hidden,
            },
        };

        cx.layout.set_overflow(node, overflow);
        cx.layout.set_flex(
            node,
            FlexStyle {
                direction: self.direction,
                ..Default::default()
            },
        );

        let mut widget = P::Scroll::build(
            &mut cx.platform,
            contents.widget.widget(),
        );

        widget.set_direction(&mut cx.platform, self.direction);

        let view_id = ViewId::next();
        cx.register(view_id);

        let proxy = cx.proxy();
        widget.set_on_scroll(&mut cx.platform, move |x, y| {
            proxy.message(Message::new(
                ScrollMessage(x, y),
                view_id,
            ));
        });

        let pod = Pod::new(node, widget);
        let state = ScrollState {
            view_id,
            state,
            direction: self.direction,
            layout: self.layout,
            scroll_allocation: None,
            content_allocation: None,
            on_scroll: self.on_scroll,
        };

        (pod, (contents, state))
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        (contents, state): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        if state.layout != self.layout {
            state.layout = self.layout;
            cx.layout.set_layout(*element.node, self.layout);
        }

        if state.direction != self.direction {
            state.direction = self.direction;

            let overflow = match self.direction {
                Direction::Row => Size {
                    width:  Overflow::Hidden,
                    height: Overflow::Visible,
                },

                Direction::Column => Size {
                    width:  Overflow::Visible,
                    height: Overflow::Hidden,
                },
            };

            cx.layout.set_overflow(*element.node, overflow);
            cx.layout.set_flex(
                *element.node,
                FlexStyle {
                    direction: self.direction,
                    ..Default::default()
                },
            );

            (element.widget).set_direction(&mut cx.platform, self.direction);
        }

        state.on_scroll = self.on_scroll;

        let pod = contents.as_mut(*element.node, 0, element.widget, 0);
        self.contents.rebuild(pod, &mut state.state, cx, data);
    }

    fn message(
        element: Mut<'_, Self::Element>,
        (contents, state): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get() {
            if let Some(allocation) = cx.layout.get_allocation(*element.node)
                && state.scroll_allocation != Some(allocation)
            {
                state.scroll_allocation = Some(allocation);
                element.widget.set_content_size(
                    &mut cx.platform,
                    allocation.content_size.width,
                    allocation.content_size.height,
                );
            }

            if let Some(allocation) = cx.layout.get_allocation(contents.node)
                && state.content_allocation != Some(allocation)
            {
                state.content_allocation = Some(allocation);
                element.widget.set_content_layout(
                    &mut cx.platform,
                    allocation.x,
                    allocation.y,
                    allocation.size.width,
                    allocation.size.height,
                );
            }
        }

        if let Some(ScrollMessage(x, y)) = message.take_targeted(state.view_id) {
            let scroll = match state.direction {
                Direction::Row => x,
                Direction::Column => y,
            };

            return (state.on_scroll)(data, scroll);
        }

        let pod = contents.as_mut(*element.node, 0, element.widget, 0);
        V::message(pod, &mut state.state, cx, data, message)
    }

    fn teardown(element: Self::Element, (contents, state): Self::State, cx: &mut Context<P>) {
        V::teardown(contents, state.state, cx);
        element.widget.teardown(&mut cx.platform);
        cx.unregister(state.view_id);
    }
}

#[allow(clippy::type_complexity)]
pub struct ScrollState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    view_id:            ViewId,
    state:              V::State,
    direction:          Direction,
    layout:             LayoutStyle,
    scroll_allocation:  Option<Allocation>,
    content_allocation: Option<Allocation>,
    on_scroll:          Box<dyn FnMut(&mut T, f32) -> Action>,
}
