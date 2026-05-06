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
