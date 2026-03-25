use std::time::Duration;

use ori::ViewId;

/// An event in the lifecycle of an application.
#[derive(Clone, Debug)]
pub enum Lifecycle {
    /// A frame has been drawn.
    Animate(Duration),

    /// Layout has been (re)computed.
    Layout,
}

/// A request regarding layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRequest {
    /// Recompute layout of contents.
    Layout,
}

/// A request regarding modals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ModalRequest {
    /// Open a modal.
    Open {
        /// Id of the modal to open.
        id: ViewId,
    },

    /// Close a modal,
    Close {
        /// Id of the modal to close.
        id: ViewId,
    },
}

/// A request regarding animation.
///
/// As long as more `start` requests than `stop` requests have been received, request animation
/// frames and sent [`Lifecycle::Animate`] events to contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimateRequest {
    /// Start requesting animation frames.
    Start,

    /// Stop requesting animation frames.
    Stop,
}
