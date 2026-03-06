use crate::Color;

/// Box shadow.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Shadow {
    /// Color of the shadow.
    pub color: Color,

    /// Offset in the `x` direction.
    pub offset_x: f32,

    /// Offset in the `y` direction.
    pub offset_y: f32,

    /// Blur radius.
    pub radius: f32,

    /// Spread radius.
    pub spread: f32,
}
