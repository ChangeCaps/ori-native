use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Allocation, Context, Direction, FlexStyle, Layout, LayoutStyle, Lifecycle, NativeWidget,
    Overflow, Platform, Pod, Size, WidgetView, native::NativeScroll,
};

/// [`View`] of a horizontal scroll area.
pub fn hscroll<V>(contents: V) -> Scroll<V> {
    Scroll::new(contents, Direction::Row)
}

/// [`View`] of a vertical scroll area.
pub fn vscroll<V>(contents: V) -> Scroll<V> {
    Scroll::new(contents, Direction::Column)
}

/// [`View`] of a scroll area.
pub struct Scroll<V> {
    contents:  V,
    direction: Direction,
    layout:    LayoutStyle,
}

impl<V> Scroll<V> {
    /// Create new [`Scroll`].
    pub fn new(contents: V, direction: Direction) -> Self {
        Self {
            contents,
            direction,
            layout: LayoutStyle::default(),
        }
    }
}

impl<V> Layout for Scroll<V> {
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle {
        &mut self.layout
    }
}

impl<V> ViewMarker for Scroll<V> {}
impl<P, T, V> View<Context<P>, T> for Scroll<V>
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
            Direction::Row | Direction::RowReverse => Size {
                width:  Overflow::Hidden,
                height: Overflow::Visible,
            },

            Direction::Column | Direction::ColumnReverse => Size {
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

        let pod = Pod::new(node, widget);
        let state = ScrollState {
            state,
            direction: self.direction,
            layout: self.layout,
            allocation: Default::default(),
            content_allocation: Default::default(),
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
                Direction::Row | Direction::RowReverse => Size {
                    width:  Overflow::Hidden,
                    height: Overflow::Visible,
                },

                Direction::Column | Direction::ColumnReverse => Size {
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
        if let Some(Lifecycle::Layout) = message.get()
            && let Some(allocation) = cx.layout.get_allocation(*element.node)
            && state.allocation != Some(allocation)
        {
            state.allocation = Some(allocation);
            element.widget.set_content_size(
                &mut cx.platform,
                allocation.content_size.width,
                allocation.content_size.height,
            );
        }

        if let Some(Lifecycle::Layout) = message.get()
            && let Some(allocation) = cx.layout.get_allocation(contents.node)
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

        let pod = contents.as_mut(*element.node, 0, element.widget, 0);
        V::message(pod, &mut state.state, cx, data, message)
    }

    fn teardown(element: Self::Element, (contents, state): Self::State, cx: &mut Context<P>) {
        V::teardown(contents, state.state, cx);
        element.widget.teardown(&mut cx.platform);
    }
}

pub struct ScrollState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    state:              V::State,
    direction:          Direction,
    layout:             LayoutStyle,
    allocation:         Option<Allocation>,
    content_allocation: Option<Allocation>,
}
