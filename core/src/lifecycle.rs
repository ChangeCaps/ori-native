/// A request regarding layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRequest {
    /// Recompute layout of contents.
    Layout,
}

/// A request regarding animation.
///
/// As long as more `start` requests than `stop` requests have been received, request animation
/// frames and animate contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimateRequest {
    /// Start requesting animation frames.
    Start,

    /// Stop requesting animation frames.
    Stop,
}
