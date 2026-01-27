pub const fn percent(percent: f32) -> Dimension {
    Dimension::Percent(percent / 100.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension {
    Length(f32),
    Percent(f32),
    Auto,
}

impl From<f32> for Dimension {
    fn from(length: f32) -> Self {
        Dimension::Length(length)
    }
}

impl Dimension {
    fn into_taffy(self) -> taffy::Dimension {
        match self {
            Dimension::Length(x) => taffy::Dimension::length(x),
            Dimension::Percent(x) => taffy::Dimension::percent(x),
            Dimension::Auto => taffy::Dimension::auto(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Align {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
    FlexStart,
    FlexEnd,
}

impl Align {
    fn into_taffy(self) -> taffy::AlignItems {
        match self {
            Align::Start => taffy::AlignItems::Start,
            Align::Center => taffy::AlignItems::Center,
            Align::End => taffy::AlignItems::End,
            Align::Baseline => taffy::AlignItems::Baseline,
            Align::Stretch => taffy::AlignItems::Stretch,
            Align::FlexStart => taffy::AlignItems::FlexStart,
            Align::FlexEnd => taffy::AlignItems::FlexEnd,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Justify {
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceEvenly,
    SpaceAround,
    FlexStart,
    FlexEnd,
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
            Justify::FlexStart => taffy::AlignContent::FlexStart,
            Justify::FlexEnd => taffy::AlignContent::FlexEnd,
        }
    }
}

pub trait Layout: Sized {
    fn style_mut(&mut self) -> &mut taffy::Style;

    fn size(mut self, width: impl Into<Dimension>, height: impl Into<Dimension>) -> Self {
        self.style_mut().size.width = width.into().into_taffy();
        self.style_mut().size.height = height.into().into_taffy();
        self
    }

    fn min_size(mut self, width: impl Into<Dimension>, height: impl Into<Dimension>) -> Self {
        self.style_mut().min_size.width = width.into().into_taffy();
        self.style_mut().min_size.height = height.into().into_taffy();
        self
    }

    fn max_size(mut self, width: impl Into<Dimension>, height: impl Into<Dimension>) -> Self {
        self.style_mut().max_size.width = width.into().into_taffy();
        self.style_mut().max_size.height = height.into().into_taffy();
        self
    }
}

pub trait FlexContainer: Layout {
    fn align_items(mut self, align: Align) -> Self {
        self.style_mut().align_items = Some(align.into_taffy());
        self
    }

    fn align_contents(mut self, justify: Justify) -> Self {
        self.style_mut().align_content = Some(justify.into_taffy());
        self
    }

    fn justify_contents(mut self, justify: Justify) -> Self {
        self.style_mut().justify_content = Some(justify.into_taffy());
        self
    }
}

pub trait FlexItem: Layout {
    fn flex_grow(mut self, amount: f32) -> Self {
        self.style_mut().flex_grow = amount;
        self
    }

    fn flex_shrink(mut self, amount: f32) -> Self {
        self.style_mut().flex_shrink = amount;
        self
    }

    fn flex(self, amount: f32) -> Self {
        self.flex_grow(amount).flex_shrink(amount)
    }
}
