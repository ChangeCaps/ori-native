/// Behaviour of overflowing contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Overflow {
    /// The overflowing contents is visible.
    Visible,

    /// The overflowing contents is hidden.
    Hidden,
}
