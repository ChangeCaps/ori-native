use crate::Color;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Shadow {
    pub color:    Color,
    pub offset_x: f32,
    pub offset_y: f32,
    pub radius:   f32,
    pub spread:   f32,
}
