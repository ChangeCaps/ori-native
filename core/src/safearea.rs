use crate::Sides;

/// Insets needed for system elements.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct SafeAreaInsets(pub Sides<f32>);
