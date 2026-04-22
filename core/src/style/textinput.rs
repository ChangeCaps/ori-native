/// Behaviour of newlines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Newline {
    /// Newlines are never inserted.
    None,

    /// Newlines are inserted when `enter` is pressed.
    Enter,

    /// Newlines are inserted when `enter` is pressed while `shift` is held.
    ShiftEnter,
}
