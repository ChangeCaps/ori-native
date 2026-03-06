use ori::Proxied;

/// A native platform, e.g. windows or gtk4.
pub trait Platform: Proxied + Sized + 'static {
    /// The base widget of this platform.
    type Widget;

    /// Quit the application.
    fn quit(&mut self);
}
