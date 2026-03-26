use std::{convert::Infallible, mem};

use crate::{
    Align, AutoLength, BorderStyle, Direction, FlexStyle, Justify, LayoutStyle, Length, Overflow,
    Position, Sides, Size,
};

/// A leaf in the layout tree.
pub trait Measurable<P>: 'static {
    /// Compute the size for the given constraints.
    fn measure(
        &mut self,
        platform: &mut P,
        known_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32>;
}

impl<P> Measurable<P> for Infallible {
    fn measure(
        &mut self,
        _platform: &mut P,
        _known_size: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
    ) -> Size<f32> {
        unreachable!()
    }
}

/// Available space in a given dimension.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AvailableSpace {
    /// A specific length in logical pixels.
    Definite(f32),

    /// The minimum size of contents.
    MinContent,

    /// The maximum size of contents.
    MaxContent,
}

/// The computed size of a [`LayoutNode`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Allocation {
    /// The x coordinate relative to the parent.
    pub x: f32,

    /// The y coordinate relative to the parent.
    pub y: f32,

    /// The allocated size.
    pub size: Size<f32>,

    /// The size of the contents.
    pub content_size: Size<f32>,

    /// The border widths.
    pub border: Sides<f32>,
}

/// Id of a node in the [`LayoutTree`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LayoutNode {
    id: taffy::NodeId,
}

/// The layout tree of an application.
pub struct LayoutTree<P> {
    request_layout: Option<Box<dyn FnOnce()>>,
    tree:           taffy::TaffyTree<Box<dyn Measurable<P>>>,
}

impl<P> Default for LayoutTree<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> LayoutTree<P> {
    /// Create new [`LayoutTree`].
    pub fn new() -> Self {
        Self {
            request_layout: None,
            tree:           taffy::TaffyTree::new(),
        }
    }

    /// Set the layout request callback.
    pub fn set_request_layout(
        &mut self,
        request_layout: Option<Box<dyn FnOnce()>>,
    ) -> Option<Box<dyn FnOnce()>> {
        mem::replace(&mut self.request_layout, request_layout)
    }

    /// Request a layout.
    pub fn request_layout(&mut self) {
        if let Some(request_layout) = self.request_layout.take() {
            request_layout();
        }
    }

    /// Get the computed layout of a layout node.
    pub fn get_allocation(&self, node: LayoutNode) -> Option<Allocation> {
        let layout = self.tree.layout(node.id).ok()?;

        Some(Allocation {
            x: layout.location.x,
            y: layout.location.y,

            size: Size {
                width:  layout.size.width,
                height: layout.size.height,
            },

            content_size: Size {
                width:  layout.content_size.width,
                height: layout.content_size.height,
            },

            border: Sides {
                top:    layout.border.top,
                right:  layout.border.right,
                bottom: layout.border.bottom,
                left:   layout.border.left,
            },
        })
    }

    /// Compute the layout of a layout tree with `node` as its root.
    pub fn compute_layout(
        &mut self,
        platform: &mut P,
        node: LayoutNode,
        space: Size<AvailableSpace>,
    ) where
        P: 'static,
    {
        let _ = self.tree.compute_layout_with_measure(
            node.id,
            taffy::Size {
                width:  Self::into_available_space(space.width),
                height: Self::into_available_space(space.height),
            },
            |known_size, space, _node, context, _style| match context {
                Some(leaf) => {
                    let size = leaf.measure(
                        platform,
                        Size {
                            width:  known_size.width,
                            height: known_size.height,
                        },
                        Size {
                            width:  Self::from_available_space(space.width),
                            height: Self::from_available_space(space.height),
                        },
                    );

                    taffy::Size {
                        width:  size.width,
                        height: size.height,
                    }
                }

                None => taffy::Size::ZERO,
            },
        );
    }

    /// Create a new layout node.
    pub fn add_node(&mut self, children: &[LayoutNode]) -> LayoutNode {
        let id = self
            .tree
            .new_with_children(taffy::Style::DEFAULT, &[])
            .expect("should never fail");

        for child in children {
            let _ = self.tree.add_child(id, child.id);
        }

        LayoutNode { id }
    }

    /// Create a new layout leaf.
    pub fn add_leaf<T>(&mut self, measurable: T) -> LayoutNode
    where
        T: Measurable<P> + 'static,
    {
        let id = (self.tree)
            .new_leaf_with_context(
                taffy::Style::DEFAULT,
                Box::new(measurable),
            )
            .expect("should never fail");

        LayoutNode { id }
    }

    /// Insert a child at `index` in a layout node.
    pub fn insert_child(&mut self, parent: LayoutNode, index: usize, child: LayoutNode) {
        self.request_layout();
        let _ = self.tree.insert_child_at_index(parent.id, index, child.id);
    }

    /// Replace the child at `index` in a layout node.
    pub fn replace_child(&mut self, parent: LayoutNode, index: usize, child: LayoutNode) {
        self.request_layout();
        let _ = self.tree.replace_child_at_index(parent.id, index, child.id);
    }

    /// Remove a layout node.
    pub fn remove_node(&mut self, node: LayoutNode) {
        self.request_layout();
        let _ = self.tree.remove(node.id);
    }

    /// Remove the child at `index` from a layout node.
    pub fn remove_child(&mut self, node: LayoutNode, index: usize) {
        self.request_layout();
        let _ = self.tree.remove_child_at_index(node.id, index);
    }

    /// Set the layout style of a layout node.
    pub fn set_layout(&mut self, node: LayoutNode, style: LayoutStyle) {
        let Ok(mut layout) = self.tree.style(node.id).cloned() else {
            return;
        };

        layout.position = style.position.into_taffy();
        layout.justify_self = style.justify_self.map(|align| align.into_taffy());
        layout.align_self = style.align_self.map(|align| align.into_taffy());
        layout.flex_shrink = style.flex_shrink;
        layout.flex_grow = style.flex_grow;
        layout.flex_basis = style.flex_basis.into_taffy_dimension();

        layout.margin = taffy::Rect {
            top:    style.margin.top.into_taffy_length_auto(),
            right:  style.margin.right.into_taffy_length_auto(),
            bottom: style.margin.bottom.into_taffy_length_auto(),
            left:   style.margin.left.into_taffy_length_auto(),
        };

        layout.inset = taffy::Rect {
            top:    style.inset.top.into_taffy_length_auto(),
            right:  style.inset.right.into_taffy_length_auto(),
            bottom: style.inset.bottom.into_taffy_length_auto(),
            left:   style.inset.left.into_taffy_length_auto(),
        };

        layout.size = taffy::Size {
            width:  style.size.width.into_taffy_dimension(),
            height: style.size.height.into_taffy_dimension(),
        };

        layout.min_size = taffy::Size {
            width:  style.min_size.width.into_taffy_dimension(),
            height: style.min_size.height.into_taffy_dimension(),
        };

        layout.max_size = taffy::Size {
            width:  style.max_size.width.into_taffy_dimension(),
            height: style.max_size.height.into_taffy_dimension(),
        };

        self.request_layout();
        let _ = self.tree.set_style(node.id, layout);
    }

    /// Set the border style of a layout node.
    pub fn set_border(&mut self, node: LayoutNode, style: BorderStyle) {
        let Ok(mut layout) = self.tree.style(node.id).cloned() else {
            return;
        };

        layout.border = taffy::Rect {
            top:    style.width.top.into_taffy(),
            right:  style.width.right.into_taffy(),
            bottom: style.width.bottom.into_taffy(),
            left:   style.width.left.into_taffy(),
        };

        self.request_layout();
        let _ = self.tree.set_style(node.id, layout);
    }

    /// Set the padding of a layout node.
    pub fn set_padding(&mut self, node: LayoutNode, padding: Sides<Length>) {
        let Ok(mut layout) = self.tree.style(node.id).cloned() else {
            return;
        };

        layout.padding = taffy::Rect {
            top:    padding.top.into_taffy(),
            right:  padding.right.into_taffy(),
            bottom: padding.bottom.into_taffy(),
            left:   padding.left.into_taffy(),
        };

        self.request_layout();
        let _ = self.tree.set_style(node.id, layout);
    }

    /// Set the overflow of a layout node.
    pub fn set_overflow(&mut self, node: LayoutNode, overflow: Size<Overflow>) {
        let Ok(mut layout) = self.tree.style(node.id).cloned() else {
            return;
        };

        layout.overflow = taffy::Point {
            x: Self::into_overflow(overflow.width),
            y: Self::into_overflow(overflow.height),
        };

        self.request_layout();
        let _ = self.tree.set_style(node.id, layout);
    }

    /// Set the flex parameters of a layout node.
    pub fn set_flex(&mut self, node: LayoutNode, flex: FlexStyle) {
        let Ok(mut layout) = self.tree.style(node.id).cloned() else {
            return;
        };

        layout.flex_direction = match flex.direction {
            Direction::Row => taffy::FlexDirection::Row,
            Direction::Column => taffy::FlexDirection::Column,
            Direction::RowReverse => taffy::FlexDirection::RowReverse,
            Direction::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        };

        layout.gap = taffy::Size {
            width:  flex.gap.width.into_taffy(),
            height: flex.gap.height.into_taffy(),
        };

        layout.justify_content = flex.justify_content.map(Justify::into_taffy);
        layout.align_items = flex.align_items.map(Align::into_taffy);

        self.request_layout();
        let _ = self.tree.set_style(node.id, layout);
    }

    /// Set the measure of a layout.
    pub fn set_measure<T>(&mut self, node: LayoutNode, leaf: T)
    where
        T: Measurable<P> + 'static,
    {
        self.request_layout();
        let _ = self.tree.set_node_context(node.id, Some(Box::new(leaf)));
    }

    fn into_overflow(overflow: Overflow) -> taffy::Overflow {
        match overflow {
            Overflow::Visible => taffy::Overflow::Visible,
            Overflow::Hidden => taffy::Overflow::Hidden,
        }
    }

    fn into_available_space(space: AvailableSpace) -> taffy::AvailableSpace {
        match space {
            AvailableSpace::Definite(length) => taffy::AvailableSpace::Definite(length),
            AvailableSpace::MinContent => taffy::AvailableSpace::MinContent,
            AvailableSpace::MaxContent => taffy::AvailableSpace::MaxContent,
        }
    }

    fn from_available_space(space: taffy::AvailableSpace) -> AvailableSpace {
        match space {
            taffy::AvailableSpace::Definite(length) => AvailableSpace::Definite(length),
            taffy::AvailableSpace::MinContent => AvailableSpace::MinContent,
            taffy::AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
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
