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
    Horizontal,

    /// Vertical or column.
    Vertical,
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

/// A trait for views that can style its layout.
pub trait Layoutable: Sized {
    /// Get a mutable reference to the layout style.
    fn style_mut(&mut self) -> &mut taffy::Style;

    /// Set the positioning strategy.
    fn position(mut self, position: Position) -> Self {
        self.style_mut().position = position.into_taffy();
        self
    }

    /// Set the inset from all sides.
    fn inset(self, inset: impl Into<AutoLength>) -> Self {
        let inset = inset.into();
        self.inset_all(inset, inset, inset, inset)
    }

    /// Set the inset from the top.
    fn top(mut self, inset: impl Into<AutoLength>) -> Self {
        self.style_mut().inset.top = inset.into().into_taffy_length_auto();
        self
    }

    /// Set the inset from the right.
    fn right(mut self, inset: impl Into<AutoLength>) -> Self {
        self.style_mut().inset.right = inset.into().into_taffy_length_auto();
        self
    }

    /// Set the inset from the bottom.
    fn bottom(mut self, inset: impl Into<AutoLength>) -> Self {
        self.style_mut().inset.bottom = inset.into().into_taffy_length_auto();
        self
    }

    /// Set the inset from the left.
    fn left(mut self, inset: impl Into<AutoLength>) -> Self {
        self.style_mut().inset.left = inset.into().into_taffy_length_auto();
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
        self.style_mut().size.width = width.into().into_taffy_dimension();
        self
    }

    /// Set the `height`.
    fn height(mut self, height: impl Into<AutoLength>) -> Self {
        self.style_mut().size.height = height.into().into_taffy_dimension();
        self
    }

    /// Set the minimum `width` and `height`.
    fn min_size(self, min_width: impl Into<AutoLength>, min_height: impl Into<AutoLength>) -> Self {
        self.min_width(min_width).min_height(min_height)
    }

    /// Set the minimum `width`.
    fn min_width(mut self, min_width: impl Into<AutoLength>) -> Self {
        self.style_mut().min_size.width = min_width.into().into_taffy_dimension();
        self
    }

    /// Set the minimum `height`.
    fn min_height(mut self, min_height: impl Into<AutoLength>) -> Self {
        self.style_mut().min_size.height = min_height.into().into_taffy_dimension();
        self
    }

    /// Set the maximum `width` and `height`.
    fn max_size(self, max_width: impl Into<AutoLength>, max_height: impl Into<AutoLength>) -> Self {
        self.max_width(max_width).max_height(max_height)
    }

    /// Set the maximum `width`.
    fn max_width(mut self, max_width: impl Into<AutoLength>) -> Self {
        self.style_mut().max_size.width = max_width.into().into_taffy_dimension();
        self
    }

    /// Set the maximum `height`.
    fn max_height(mut self, max_height: impl Into<AutoLength>) -> Self {
        self.style_mut().max_size.height = max_height.into().into_taffy_dimension();
        self
    }

    /// Set the margin on all sides.
    fn margin(self, width: impl Into<AutoLength>) -> Self {
        let width = width.into();
        self.margin_all(width, width, width, width)
    }

    /// Set the margin on the top.
    fn margin_top(mut self, width: impl Into<AutoLength>) -> Self {
        self.style_mut().margin.top = width.into().into_taffy_length_auto();
        self
    }

    /// Set the margin on the right.
    fn margin_right(mut self, width: impl Into<AutoLength>) -> Self {
        self.style_mut().margin.right = width.into().into_taffy_length_auto();
        self
    }

    /// Set the margin on the bottom.
    fn margin_bottom(mut self, width: impl Into<AutoLength>) -> Self {
        self.style_mut().margin.bottom = width.into().into_taffy_length_auto();
        self
    }

    /// Set the margin on the left.
    fn margin_left(mut self, width: impl Into<AutoLength>) -> Self {
        self.style_mut().margin.left = width.into().into_taffy_length_auto();
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
        self.style_mut().flex_grow = amount;
        self
    }

    /// Set the flex shrinkage factor.
    fn flex_shrink(mut self, amount: f32) -> Self {
        self.style_mut().flex_shrink = amount;
        self
    }
}

/// A trait for views with borders.
pub trait Bordered: Layoutable {
    /// Set the border width on all sides.
    fn border(self, width: impl Into<Length>) -> Self {
        let width = width.into();
        self.border_all(width, width, width, width)
    }

    /// Set the border width on the top.
    fn border_top(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().border.top = width.into().into_taffy();
        self
    }

    /// Set the border width on the right.
    fn border_right(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().border.right = width.into().into_taffy();
        self
    }

    /// Set the border width on the bottom.
    fn border_bottom(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().border.bottom = width.into().into_taffy();
        self
    }

    /// Set the border width on the left.
    fn border_left(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().border.left = width.into().into_taffy();
        self
    }

    /// Set the border width on all individually.
    fn border_all(
        self,
        top: impl Into<Length>,
        right: impl Into<Length>,
        bottom: impl Into<Length>,
        left: impl Into<Length>,
    ) -> Self {
        self.border_top(top)
            .border_right(right)
            .border_bottom(bottom)
            .border_left(left)
    }
}

/// A trait for container views.
pub trait Container: Layoutable {
    /// Set the padding on all sides.
    fn padding(self, width: impl Into<Length>) -> Self {
        let width = width.into();
        self.padding_all(width, width, width, width)
    }

    /// Set the padding on the top.
    fn padding_top(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().padding.top = width.into().into_taffy();
        self
    }

    /// Set the padding on the right.
    fn padding_right(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().padding.right = width.into().into_taffy();
        self
    }

    /// Set the padding on the bottom.
    fn padding_bottom(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().padding.bottom = width.into().into_taffy();
        self
    }

    /// Set the padding on the left.
    fn padding_left(mut self, width: impl Into<Length>) -> Self {
        self.style_mut().padding.left = width.into().into_taffy();
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

/// A trait for flex container views.
pub trait FlexContainer: Container {
    /// Set the gap between items.
    fn gap(mut self, gap: impl Into<Length>) -> Self {
        self.style_mut().gap.width = gap.into().into_taffy();
        self.style_mut().gap.height = self.style_mut().gap.width;
        self
    }

    /// Set the alignment of items.
    fn align_items(mut self, align: Align) -> Self {
        self.style_mut().align_items = Some(align.into_taffy());
        self
    }

    /// Set the alignment of contents.
    fn align_contents(mut self, justify: Justify) -> Self {
        self.style_mut().align_content = Some(justify.into_taffy());
        self
    }

    /// Set the justification of contents.
    fn justify_contents(mut self, justify: Justify) -> Self {
        self.style_mut().justify_content = Some(justify.into_taffy());
        self
    }
}

impl AutoLength {
    fn into_taffy_dimension(self) -> taffy::Dimension {
        match self {
            AutoLength::Length(x) => taffy::Dimension::length(x),
            AutoLength::Fract(x) => taffy::Dimension::percent(x),
            AutoLength::Auto => taffy::Dimension::auto(),
        }
    }

    fn into_taffy_length_auto(self) -> taffy::LengthPercentageAuto {
        match self {
            AutoLength::Length(x) => taffy::LengthPercentageAuto::length(x),
            AutoLength::Fract(x) => taffy::LengthPercentageAuto::percent(x),
            AutoLength::Auto => taffy::LengthPercentageAuto::auto(),
        }
    }
}

impl Length {
    fn into_taffy(self) -> taffy::LengthPercentage {
        match self {
            Length::Length(x) => taffy::LengthPercentage::length(x),
            Length::Fract(x) => taffy::LengthPercentage::percent(x),
        }
    }
}

impl Position {
    fn into_taffy(self) -> taffy::Position {
        match self {
            Position::Relative => taffy::Position::Relative,
            Position::Absolute => taffy::Position::Absolute,
        }
    }
}

impl Align {
    fn into_taffy(self) -> taffy::AlignItems {
        match self {
            Align::Start => taffy::AlignItems::Start,
            Align::Center => taffy::AlignItems::Center,
            Align::End => taffy::AlignItems::End,
            Align::Baseline => taffy::AlignItems::Baseline,
            Align::Stretch => taffy::AlignItems::Stretch,
        }
    }
}

impl Justify {
    fn into_taffy(self) -> taffy::AlignContent {
        match self {
            Justify::Start => taffy::AlignContent::Start,
            Justify::Center => taffy::AlignContent::Center,
            Justify::End => taffy::AlignContent::End,
            Justify::Stretch => taffy::AlignContent::Stretch,
            Justify::SpaceBetween => taffy::AlignContent::SpaceBetween,
            Justify::SpaceEvenly => taffy::AlignContent::SpaceEvenly,
            Justify::SpaceAround => taffy::AlignContent::SpaceAround,
        }
    }
}
