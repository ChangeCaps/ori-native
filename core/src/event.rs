/// The state of a press.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pointer {
    /// The `x` coordinate on the pointer.
    pub x: f32,

    /// The `y` coordinate on the pointer.
    pub y: f32,
}

/// An event that can happen to a [`pressable`](crate::views::pressable).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PressableEvent {
    /// The pointer was pressed.
    Pressed(Pointer),

    /// The pointer was released.
    Released(Pointer),

    /// The press was cancelled.
    Cancelled(Pointer),

    /// The pointer moved.
    Moved(Pointer),

    /// The view changed hovered state.
    Hovered(bool),

    /// The view changed focused state.
    Focused(bool),
}
