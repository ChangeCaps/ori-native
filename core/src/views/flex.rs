use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Border, Color, Container, Context, Direction, FlexContainer, Layout, Lifecycle, Overflow,
    Platform, Pod, Shadow, element::WidgetViewSeq, native::WrappedGroup,
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
    contents:         V,
    layout:           taffy::Style,
    background_color: Color,
    border_color:     Color,
    corner_radii:     [f32; 4],
    overflow:         Overflow,
    shadow:           Shadow,
    hardware_layer:   bool,
}

impl<V> Flex<V> {
    /// Create new [`Flex`].
    pub fn new(contents: V) -> Self {
        Self {
            contents,
            layout: taffy::Style {
                display: taffy::Display::Flex,
                ..Default::default()
            },
            background_color: Color::TRANSPARENT,
            border_color: Color::TRANSPARENT,
            corner_radii: [0.0; 4],
            overflow: Overflow::Visible,
            shadow: Shadow::default(),
            hardware_layer: false,
        }
    }

    /// Set the flex direction.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.flex_direction = match direction {
            Direction::Row => taffy::FlexDirection::Row,
            Direction::Column => taffy::FlexDirection::Column,
            Direction::RowReverse => taffy::FlexDirection::RowReverse,
            Direction::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        };

        self
    }

    /// Reverse the direction.
    pub fn reverse(mut self) -> Self {
        self.layout.flex_direction = match self.layout.flex_direction {
            taffy::FlexDirection::Row => taffy::FlexDirection::RowReverse,
            taffy::FlexDirection::Column => taffy::FlexDirection::ColumnReverse,
            taffy::FlexDirection::RowReverse => taffy::FlexDirection::Row,
            taffy::FlexDirection::ColumnReverse => taffy::FlexDirection::Column,
        };

        self
    }

    /// Set the background color.
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set the overflow behaviour.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        let taffy = match overflow {
            Overflow::Visible => taffy::Overflow::Visible,
            Overflow::Hidden => taffy::Overflow::Hidden,
        };

        self.overflow = overflow;
        self.layout.overflow.x = taffy;
        self.layout.overflow.y = taffy;
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
    fn get_layout_mut(&mut self) -> &mut taffy::Style {
        &mut self.layout
    }
}

impl<V> Container for Flex<V> {}
impl<V> FlexContainer for Flex<V> {}
impl<V> Border for Flex<V> {}

impl<V> ViewMarker for Flex<V> {}
impl<P, T, V> View<Context<P>, T> for Flex<V>
where
    P: Platform,
    V: WidgetViewSeq<P, T>,
{
    type Element = Pod<P, WrappedGroup<P>>;
    type State = FlexState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let node = cx.new_layout_node(self.layout, &[]);

        let mut group = WrappedGroup::new(cx);
        group.set_background_color(cx, self.background_color);
        group.set_border_color(cx, self.border_color);
        group.set_corner_radii(cx, self.corner_radii);
        group.set_overflow(cx, self.overflow);
        group.set_shadow(cx, self.shadow);
        group.set_hardware_layer(cx, self.hardware_layer);

        let state = self.contents.seq_build(&mut group.elements(node), cx, data);
        let pod = Pod::new(node, group);

        let state = FlexState {
            state,
            background_color: self.background_color,
            border_color: self.border_color,
            corner_radii: self.corner_radii,
            overflow: self.overflow,
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
        let _ = cx.set_layout_style(*element.node, self.layout);

        if state.background_color != self.background_color {
            state.background_color = self.background_color;
            (element.widget).set_background_color(cx, self.background_color);
        }

        if state.border_color != self.border_color {
            state.border_color = self.border_color;
            (element.widget).set_border_color(cx, self.border_color);
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
        let _ = cx.remove_layout_node(element.node);
    }
}

pub struct FlexState<P, T, V>
where
    P: Platform,
    V: WidgetViewSeq<P, T>,
{
    state:            V::State,
    background_color: Color,
    border_color:     Color,
    corner_radii:     [f32; 4],
    overflow:         Overflow,
    shadow:           Shadow,
    hardware_layer:   bool,
}
