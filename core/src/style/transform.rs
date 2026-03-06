/// An affine transformation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    /// Offset in the `x` direction.
    pub offset_x: f32,

    /// Offset in the `y` direction.
    pub offset_y: f32,

    /// Rotation in degrees.
    pub rotation: f32,

    /// Scaling along the `x` axis.
    pub scale_x: f32,

    /// Scaling along the `y` axis.
    pub scale_y: f32,
}

impl Default for Affine {
    fn default() -> Self {
        Self::new()
    }
}

impl Affine {
    /// Create new identity [`Affine`].
    pub const fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            rotation: 0.0,
            scale_x:  1.0,
            scale_y:  1.0,
        }
    }
}
