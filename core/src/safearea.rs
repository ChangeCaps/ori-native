/// Insets needed for system elements.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct SafeAreaInsets {
    /// The inset from the top of the window.
    pub top: f32,

    /// The inset from the right of the window.
    pub right: f32,

    /// The inset from the bottom of the window.
    pub bottom: f32,

    /// The inset from the left of the window.
    pub left: f32,
}
