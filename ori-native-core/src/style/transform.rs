#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub offset_x: f32,
    pub offset_y: f32,
    pub rotation: f32,
    pub scale_x:  f32,
    pub scale_y:  f32,
}

impl Default for Affine {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            rotation: 0.0,
            scale_x:  1.0,
            scale_y:  1.0,
        }
    }
}
