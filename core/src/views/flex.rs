use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Align, Border, BorderStyle, Color, Context, Direction, Justify, Layout, LayoutStyle, Length,
    Lifecycle, Overflow, Padding, Platform, Pod, Shadow, Sides, Size, WidgetViewSeq, native::Group,
};

/// Container [`View`] with flexbox layout.
pub fn flex<V>(contents: V) -> Flex<V> {
    Flex::new(contents)
}

/// [`View`] of a flex row.
pub fn row<V>(contents: V) -> Flex<V> {
    Flex::new(contents).direction(Direction::Row)
}

/// [`View`] of a flex column.
pub fn column<V>(contents: V) -> Flex<V> {
    Flex::new(contents).direction(Direction::Column)
}

/// [`View`] of a flex container.
pub struct Flex<V> {
    contents:        V,
    layout:          LayoutStyle,
    border:          BorderStyle,
    padding:         Sides<Length>,
    direction:       Direction,
    justify_content: Option<Justify>,
    align_items:     Option<Align>,
    gap:             Length,
    background:      Color,
    corner_radii:    [f32; 4],
    overflow:        Overflow,
    shadow:          Shadow,
    hardware_layer:  bool,
}

impl<V> Flex<V> {
    /// Create new [`Flex`].
    pub fn new(contents: V) -> Self {
        Self {
            contents,
            layout: LayoutStyle::default(),
            border: BorderStyle::default(),
            padding: Sides::all(Length::Length(0.0)),
            direction: Direction::Row,
            justify_content: None,
            align_items: None,
            gap: Length::Length(0.0),
            background: Color::TRANSPARENT,
            corner_radii: [0.0; 4],
            overflow: Overflow::Visible,
            shadow: Shadow::default(),
            hardware_layer: false,
        }
    }

    /// Set the flex direction.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Reverse the direction.
    pub fn reverse(mut self) -> Self {
        self.direction = match self.direction {
            Direction::Row => Direction::RowReverse,
            Direction::Column => Direction::ColumnReverse,
            Direction::RowReverse => Direction::Row,
            Direction::ColumnReverse => Direction::Column,
        };

        self
    }

    /// Set how contents are justified within the container.
    pub fn justify_content(mut self, justify: Justify) -> Self {
        self.justify_content = Some(justify);
        self
    }

    /// Set how items are aligned within the container.
    pub fn align_items(mut self, align: Align) -> Self {
        self.align_items = Some(align);
        self
    }

    /// Set the gap between items within the container.
    pub fn gap(mut self, gap: impl Into<Length>) -> Self {
        self.gap = gap.into();
        self
    }

    /// Set the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    /// Set the overflow behaviour.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Set the shadow.
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    /// Set the shadow color.
    pub fn shadow_color(mut self, color: Color) -> Self {
        self.shadow.color = color;
        self
    }

    /// Set the shadow offset.
    pub fn shadow_offset(mut self, dx: f32, dy: f32) -> Self {
        self.shadow.offset_x = dx;
        self.shadow.offset_y = dy;
        self
    }

    /// Set the shadow radius.
    pub fn shadow_radius(mut self, radius: f32) -> Self {
        self.shadow.radius = radius;
        self
    }

    /// Set the shadow spread.
    pub fn shadow_spread(mut self, spread: f32) -> Self {
        self.shadow.spread = spread;
        self
    }

    /// Set the radius of all corners.
    pub fn corner(self, radius: f32) -> Self {
        self.corner_all(radius, radius, radius, radius)
    }

    /// Set the radius of the top left corner.
    pub fn corner_top_left(mut self, radius: f32) -> Self {
        self.corner_radii[0] = radius;
        self
    }

    /// Set the radius of the top right corner.
    pub fn corner_top_right(mut self, radius: f32) -> Self {
        self.corner_radii[1] = radius;
        self
    }

    /// Set the radius of the bottom right corner.
    pub fn corner_bottom_right(mut self, radius: f32) -> Self {
        self.corner_radii[2] = radius;
        self
    }

    /// Set the radius of the bottom left corner.
    pub fn corner_bottom_left(mut self, radius: f32) -> Self {
        self.corner_radii[3] = radius;
        self
    }

    /// Set the radius of all corners individually.
    pub fn corner_all(
        self,
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        self.corner_top_left(top_left)
            .corner_top_right(top_right)
            .corner_bottom_right(bottom_right)
            .corner_bottom_left(bottom_left)
    }

    /// Set whether to use a hardware layer.
    ///
    /// # Platform
    ///  - `android` set the layer type of the underlying view to hardware.
    ///  - `other` not supported.
    pub fn hardware_layer(mut self, enabled: bool) -> Self {
        self.hardware_layer = enabled;
        self
    }
}

impl<V> Layout for Flex<V> {
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle {
        &mut self.layout
    }
}

impl<V> Border for Flex<V> {
    fn get_border_style_mut(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl<V> Padding for Flex<V> {
    fn get_padding_mut(&mut self) -> &mut Sides<Length> {
        &mut self.padding
    }
}

impl<V> ViewMarker for Flex<V> {}
impl<P, T, V> View<Context<P>, T> for Flex<V>
where
    P: Platform,
    V: WidgetViewSeq<P, T>,
{
    type Element = Pod<P, Group<P>>;
    type State = FlexState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let node = cx.layout.add_node(&[]);
        cx.layout.set_layout(node, self.layout);
        cx.layout.set_border(node, self.border);
        cx.layout.set_padding(node, self.padding);
        cx.layout.set_overflow(node, Size::all(self.overflow));
        cx.layout.set_flex(
            node,
            self.direction,
            self.justify_content,
            self.align_items,
            Size::all(self.gap),
        );

        let mut group = Group::new(cx);
        group.set_background(cx, self.background);
        group.set_border_color(cx, self.border.color);
        group.set_corner_radii(cx, self.corner_radii);
        group.set_overflow(cx, self.overflow);
        group.set_shadow(cx, self.shadow);
        group.set_hardware_layer(cx, self.hardware_layer);

        let state = self.contents.seq_build(&mut group.elements(node), cx, data);
        let pod = Pod::new(node, group);

        let state = FlexState {
            state,
            layout: self.layout,
            border: self.border,
            padding: self.padding,
            overflow: self.overflow,
            direction: self.direction,
            justify_content: self.justify_content,
            align_items: self.align_items,
            gap: self.gap,
            background: self.background,
            corner_radii: self.corner_radii,
            shadow: self.shadow,
            hardware_layer: self.hardware_layer,
        };

        (pod, state)
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        if state.layout != self.layout {
            state.layout = self.layout;
            (cx.layout).set_layout(*element.node, self.layout);
        }

        if state.padding != self.padding {
            state.padding = self.padding;
            (cx.layout).set_padding(*element.node, self.padding);
        }

        if state.overflow != self.overflow {
            state.overflow = self.overflow;
            (cx.layout).set_overflow(*element.node, Size::all(self.overflow));
        }

        if state.direction != self.direction
            || state.justify_content != self.justify_content
            || state.align_items != self.align_items
            || state.gap != self.gap
        {
            state.direction = self.direction;
            state.justify_content = self.justify_content;
            state.align_items = self.align_items;
            state.gap = self.gap;
            (cx.layout).set_flex(
                *element.node,
                self.direction,
                self.justify_content,
                self.align_items,
                Size::all(self.gap),
            );
        }

        if state.border != self.border {
            state.border = self.border;
            (cx.layout).set_border(*element.node, self.border);
            (element.widget).set_border_color(cx, self.border.color);
        }

        if state.background != self.background {
            state.background = self.background;
            (element.widget).set_background(cx, self.background);
        }

        if state.corner_radii != self.corner_radii {
            state.corner_radii = self.corner_radii;
            (element.widget).set_corner_radii(cx, self.corner_radii);
        }

        if state.overflow != self.overflow {
            state.overflow = self.overflow;
            (element.widget).set_overflow(cx, self.overflow);
        }

        if state.shadow != self.shadow {
            state.shadow = self.shadow;
            (element.widget).set_shadow(cx, self.shadow);
        }

        if state.hardware_layer != self.hardware_layer {
            state.hardware_layer = self.hardware_layer;
            (element.widget).set_hardware_layer(cx, self.hardware_layer);
        }

        self.contents.seq_rebuild(
            &mut element.widget.elements(*element.node),
            &mut state.state,
            cx,
            data,
        );
    }

    fn message(
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get() {
            element.widget.layout(cx, *element.node);
        }

        V::seq_message(
            &mut element.widget.elements(*element.node),
            &mut state.state,
            cx,
            data,
            message,
        )
    }

    fn teardown(mut element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        V::seq_teardown(
            &mut element.widget.elements(element.node),
            state.state,
            cx,
        );

        element.widget.teardown(cx);
        cx.layout.remove_node(element.node);
    }
}

pub struct FlexState<P, T, V>
where
    P: Platform,
    V: WidgetViewSeq<P, T>,
{
    state:           V::State,
    layout:          LayoutStyle,
    border:          BorderStyle,
    padding:         Sides<Length>,
    overflow:        Overflow,
    direction:       Direction,
    justify_content: Option<Justify>,
    align_items:     Option<Align>,
    gap:             Length,
    background:      Color,
    corner_radii:    [f32; 4],
    shadow:          Shadow,
    hardware_layer:  bool,
}
