/// Sizing mode of a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sizing {
    /// The window is resizable and the contents fit the window.
    User,

    /// The window size is determined by the contents.
    Content,
}
