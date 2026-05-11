use crate::Point;

/// A side of a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    /// The top side.
    Top,

    /// The right side.
    Right,

    /// The bottom side.
    Bottom,

    /// The left side.
    Left,
}

/// The positioning strategy for a popup.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PopupPosition {
    /// The popup will try to position itself relative to the side of its anchor.
    Relative(Side),

    /// The popup will try to position itself with the top left corner at a given point in the
    /// coordinate space of its anchor.
    Absolute(Point<f32>),
}
