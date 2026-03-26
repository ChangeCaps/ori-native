use crate::Color;

/// Fractional length.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Fract(pub f32);

/// Length with the option of `auto`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutoLength {
    /// Length in pixels.
    Length(f32),

    /// Length in fraction of parent size.
    Fract(f32),

    /// Automatic sizing length.
    Auto,
}

impl From<f32> for AutoLength {
    fn from(x: f32) -> Self {
        AutoLength::Length(x)
    }
}

impl From<Fract> for AutoLength {
    fn from(Fract(x): Fract) -> Self {
        AutoLength::Fract(x)
    }
}

/// Length that cannot be `auto`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    /// Length in pixels.
    Length(f32),

    /// Length in fraction of parent size.
    Fract(f32),
}

impl From<f32> for Length {
    fn from(x: f32) -> Self {
        Length::Length(x)
    }
}

impl From<Fract> for Length {
    fn from(Fract(x): Fract) -> Self {
        Length::Fract(x)
    }
}

/// A direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// Horizontal or row.
    Row,

    /// Vertical or column.
    Column,

    /// Horizontal or row in reverse order.
    RowReverse,

    /// Vertical or column in reverse order.
    ColumnReverse,
}

/// Alignment of contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Align {
    /// Contents is aligned towards the start.
    Start,

    /// Contents is aligned towards the center.
    Center,

    /// Contents is aligned towards the end.
    End,

    /// Contents is aligned towards the baseline.
    Baseline,

    /// Contents stretched to fill the container.
    Stretch,
}

/// Justification of contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Justify {
    /// Contents is justified towards the start.
    Start,

    /// Contents is justified towards the center.
    Center,

    /// Contents is justified towards the end.
    End,

    /// Contents is stretched to fill the container.
    Stretch,

    /// Contents is justified with even space inbetween.
    SpaceBetween,

    /// Contents is justified with evenly.
    SpaceEvenly,

    /// Contents is justified with even space around.
    SpaceAround,
}

/// Positioning within a container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Position {
    /// Offset is computed relative to the final position.
    Relative,

    /// Offset is computed relative to the container and no space is created for the item.
    Absolute,
}

/// Values for each size of a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sides<T> {
    /// The top side.
    pub top: T,

    /// The right side.
    pub right: T,

    /// The bottom side.
    pub bottom: T,

    /// The left side.
    pub left: T,
}

impl<T> Sides<T> {
    /// Create new [`Sides`] with the same value for all sides.
    pub const fn all(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            top:    value,
            right:  value,
            bottom: value,
            left:   value,
        }
    }
}

/// Values for each corner of a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Corners<T> {
    /// The top left corner.
    pub top_left: T,

    /// The top right corner.
    pub top_right: T,

    /// The bottom right corner.
    pub bottom_right: T,

    /// The bottom left corner.
    pub bottom_left: T,
}

impl<T> Corners<T> {
    /// Create new [`Corners`] with the same value for all corners.
    pub const fn all(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            top_left:     value,
            top_right:    value,
            bottom_right: value,
            bottom_left:  value,
        }
    }
}

/// Values the width and height of a size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size<T> {
    /// The width value.
    pub width: T,

    /// The height value.
    pub height: T,
}

impl<T> Size<T> {
    /// Create new [`Size`] with the same value for width and height.
    pub const fn all(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            width:  value,
            height: value,
        }
    }
}

/// The style of a layout node.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutStyle {
    /// The positioning strategy.
    pub position: Position,

    /// The justification within the container.
    pub justify_self: Option<Align>,

    /// The alignment within the container.
    pub align_self: Option<Align>,

    /// The factor by which the view will shrink.
    pub flex_shrink: f32,

    /// The factor by which the view will grow.
    pub flex_grow: f32,

    /// The margin around the view.
    pub margin: Sides<AutoLength>,

    /// The insets from the parent.
    pub inset: Sides<AutoLength>,

    /// The size of the view.
    pub size: Size<AutoLength>,

    /// The minimum size of the view.
    pub min_size: Size<AutoLength>,

    /// The maximum size of the view.
    pub max_size: Size<AutoLength>,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            position:     Position::Relative,
            justify_self: None,
            align_self:   None,
            flex_shrink:  1.0,
            flex_grow:    0.0,
            margin:       Sides::all(AutoLength::Length(0.0)),
            inset:        Sides::all(AutoLength::Auto),
            size:         Size::all(AutoLength::Auto),
            min_size:     Size::all(AutoLength::Auto),
            max_size:     Size::all(AutoLength::Auto),
        }
    }
}

/// The style of a border.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderStyle {
    /// The color.
    pub color: Color,

    /// The widths.
    pub width: Sides<Length>,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            color: Color::TRANSPARENT,
            width: Sides::all(Length::Length(0.0)),
        }
    }
}

/// The style of a flex container.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexStyle {
    /// The direction children are layed out.
    pub direction: Direction,

    /// The justification strategy.
    pub justify_content: Option<Justify>,

    /// The alignment strategy of iems.
    pub align_items: Option<Align>,

    /// The gap between items.
    pub gap: Size<Length>,
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            direction:       Direction::Row,
            justify_content: None,
            align_items:     None,
            gap:             Size::all(Length::Length(0.0)),
        }
    }
}

/// A trait for views that can style its layout.
pub trait Layout: Sized {
    /// Get a mutable reference to the layout style.
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle;

    /// Set the positioning strategy.
    fn position(mut self, position: Position) -> Self {
        self.get_layout_style_mut().position = position;
        self
    }

    /// Set how the view should be justified in the container.
    fn justify_self(mut self, justify_self: Align) -> Self {
        self.get_layout_style_mut().justify_self = Some(justify_self);
        self
    }

    /// Set how the view should be aligned in the container.
    fn align_self(mut self, align_self: Align) -> Self {
        self.get_layout_style_mut().align_self = Some(align_self);
        self
    }

    /// Set the inset from all sides.
    fn inset(self, inset: impl Into<AutoLength>) -> Self {
        let inset = inset.into();
        self.inset_all(inset, inset, inset, inset)
    }

    /// Set the inset from the top.
    fn top(mut self, inset: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().inset.top = inset.into();
        self
    }

    /// Set the inset from the right.
    fn right(mut self, inset: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().inset.right = inset.into();
        self
    }

    /// Set the inset from the bottom.
    fn bottom(mut self, inset: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().inset.bottom = inset.into();
        self
    }

    /// Set the inset from the left.
    fn left(mut self, inset: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().inset.left = inset.into();
        self
    }

    /// Set the inset from all sides individually.
    fn inset_all(
        self,
        top: impl Into<AutoLength>,
        right: impl Into<AutoLength>,
        bottom: impl Into<AutoLength>,
        left: impl Into<AutoLength>,
    ) -> Self {
        self.top(top).right(right).bottom(bottom).left(left)
    }

    /// Set the `width` and `height`.
    fn size(self, width: impl Into<AutoLength>, height: impl Into<AutoLength>) -> Self {
        self.width(width).height(height)
    }

    /// Set the `width`.
    fn width(mut self, width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().size.width = width.into();
        self
    }

    /// Set the `height`.
    fn height(mut self, height: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().size.height = height.into();
        self
    }

    /// Set the minimum `width` and `height`.
    fn min_size(self, min_width: impl Into<AutoLength>, min_height: impl Into<AutoLength>) -> Self {
        self.min_width(min_width).min_height(min_height)
    }

    /// Set the minimum `width`.
    fn min_width(mut self, min_width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().min_size.width = min_width.into();
        self
    }

    /// Set the minimum `height`.
    fn min_height(mut self, min_height: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().min_size.height = min_height.into();
        self
    }

    /// Set the maximum `width` and `height`.
    fn max_size(self, max_width: impl Into<AutoLength>, max_height: impl Into<AutoLength>) -> Self {
        self.max_width(max_width).max_height(max_height)
    }

    /// Set the maximum `width`.
    fn max_width(mut self, max_width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().max_size.width = max_width.into();
        self
    }

    /// Set the maximum `height`.
    fn max_height(mut self, max_height: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().max_size.height = max_height.into();
        self
    }

    /// Set the margin on all sides.
    fn margin(self, width: impl Into<AutoLength>) -> Self {
        let width = width.into();
        self.margin_all(width, width, width, width)
    }

    /// Set the margin on the top.
    fn margin_top(mut self, width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().margin.top = width.into();
        self
    }

    /// Set the margin on the right.
    fn margin_right(mut self, width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().margin.right = width.into();
        self
    }

    /// Set the margin on the bottom.
    fn margin_bottom(mut self, width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().margin.bottom = width.into();
        self
    }

    /// Set the margin on the left.
    fn margin_left(mut self, width: impl Into<AutoLength>) -> Self {
        self.get_layout_style_mut().margin.left = width.into();
        self
    }

    /// Set the margin on all sides individually.
    fn margin_all(
        self,
        top: impl Into<AutoLength>,
        right: impl Into<AutoLength>,
        bottom: impl Into<AutoLength>,
        left: impl Into<AutoLength>,
    ) -> Self {
        self.margin_top(top)
            .margin_right(right)
            .margin_bottom(bottom)
            .margin_left(left)
    }

    /// Set the flex factor.
    fn flex(self, amount: f32) -> Self {
        self.flex_grow(amount).flex_shrink(amount)
    }

    /// Set the flex growth factor.
    fn flex_grow(mut self, amount: f32) -> Self {
        self.get_layout_style_mut().flex_grow = amount;
        self
    }

    /// Set the flex shrinkage factor.
    fn flex_shrink(mut self, amount: f32) -> Self {
        self.get_layout_style_mut().flex_shrink = amount;
        self
    }
}

/// A trait for views with borders.
pub trait Border: Sized {
    /// Get a mutable reference to the border style.
    fn get_border_style_mut(&mut self) -> &mut BorderStyle;

    /// Set the border width and color.
    fn border(self, width: impl Into<Length>, color: Color) -> Self {
        self.border_width(width).border_color(color)
    }

    /// Set the border color.
    fn border_color(mut self, color: Color) -> Self {
        self.get_border_style_mut().color = color;
        self
    }

    /// Set the border width on all sides.
    fn border_width(self, width: impl Into<Length>) -> Self {
        let width = width.into();
        self.border_width_all(width, width, width, width)
    }

    /// Set the border width on the top, and color.
    fn border_top(self, width: impl Into<Length>, color: Color) -> Self {
        self.border_top_width(width).border_color(color)
    }

    /// Set the border width on the right, and color.
    fn border_right(self, width: impl Into<Length>, color: Color) -> Self {
        self.border_right_width(width).border_color(color)
    }

    /// Set the border width on the bottom, and color.
    fn border_bottom(self, width: impl Into<Length>, color: Color) -> Self {
        self.border_bottom_width(width).border_color(color)
    }

    /// Set the border width on the left, and color.
    fn border_left(self, width: impl Into<Length>, color: Color) -> Self {
        self.border_left_width(width).border_color(color)
    }

    /// Set the border width on the top.
    fn border_top_width(mut self, width: impl Into<Length>) -> Self {
        self.get_border_style_mut().width.top = width.into();
        self
    }

    /// Set the border width on the right.
    fn border_right_width(mut self, width: impl Into<Length>) -> Self {
        self.get_border_style_mut().width.right = width.into();
        self
    }

    /// Set the border width on the bottom.
    fn border_bottom_width(mut self, width: impl Into<Length>) -> Self {
        self.get_border_style_mut().width.bottom = width.into();
        self
    }

    /// Set the border width on the left.
    fn border_left_width(mut self, width: impl Into<Length>) -> Self {
        self.get_border_style_mut().width.left = width.into();
        self
    }

    /// Set the border width on all individually.
    fn border_width_all(
        self,
        top: impl Into<Length>,
        right: impl Into<Length>,
        bottom: impl Into<Length>,
        left: impl Into<Length>,
    ) -> Self {
        self.border_top_width(top)
            .border_right_width(right)
            .border_bottom_width(bottom)
            .border_left_width(left)
    }
}

/// A trait for container views.
pub trait Padding: Sized {
    /// Get a mutable reference to the padding.
    fn get_padding_mut(&mut self) -> &mut Sides<Length>;

    /// Set the padding on all sides.
    fn padding(self, width: impl Into<Length>) -> Self {
        let width = width.into();
        self.padding_all(width, width, width, width)
    }

    /// Set the padding on the top.
    fn padding_top(mut self, width: impl Into<Length>) -> Self {
        self.get_padding_mut().top = width.into();
        self
    }

    /// Set the padding on the right.
    fn padding_right(mut self, width: impl Into<Length>) -> Self {
        self.get_padding_mut().right = width.into();
        self
    }

    /// Set the padding on the bottom.
    fn padding_bottom(mut self, width: impl Into<Length>) -> Self {
        self.get_padding_mut().bottom = width.into();
        self
    }

    /// Set the padding on the left.
    fn padding_left(mut self, width: impl Into<Length>) -> Self {
        self.get_padding_mut().left = width.into();
        self
    }

    /// Set the padding on all individually.
    fn padding_all(
        self,
        top: impl Into<Length>,
        right: impl Into<Length>,
        bottom: impl Into<Length>,
        left: impl Into<Length>,
    ) -> Self {
        self.padding_top(top)
            .padding_right(right)
            .padding_bottom(bottom)
            .padding_left(left)
    }
}

/// A trait for flex containers.
pub trait FlexContainer: Sized {
    /// Get a mutable reference to the flex style.
    fn get_flex_style_mut(&mut self) -> &mut FlexStyle;

    /// Set the flex direction.
    fn direction(mut self, direction: Direction) -> Self {
        self.get_flex_style_mut().direction = direction;
        self
    }

    /// Reverse the direction.
    fn reverse(mut self) -> Self {
        self.get_flex_style_mut().direction = match self.get_flex_style_mut().direction {
            Direction::Row => Direction::RowReverse,
            Direction::Column => Direction::ColumnReverse,
            Direction::RowReverse => Direction::Row,
            Direction::ColumnReverse => Direction::Column,
        };

        self
    }

    /// Set how contents are justified within the container.
    fn justify_content(mut self, justify: impl Into<Option<Justify>>) -> Self {
        self.get_flex_style_mut().justify_content = justify.into();
        self
    }

    /// Set how items are aligned within the container.
    fn align_items(mut self, align: impl Into<Option<Align>>) -> Self {
        self.get_flex_style_mut().align_items = align.into();
        self
    }

    /// Set the gap between items within the container.
    fn gap(mut self, gap: impl Into<Length>) -> Self {
        let gap = gap.into();

        self.get_flex_style_mut().gap.width = gap;
        self.get_flex_style_mut().gap.height = gap;

        self
    }
}
