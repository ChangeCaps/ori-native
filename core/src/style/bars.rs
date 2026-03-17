use crate::Color;

/// Style of the status bar on android and ios.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StatusBar {
    /// The background color.
    ///
    /// # Platform
    /// - Only supported on `android`.
    pub color: Option<Color>,

    /// Whether it is shown.
    pub visible: bool,

    /// Whether the theme of the bar should be light.
    pub light: bool,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            color:   None,
            visible: true,
            light:   false,
        }
    }
}

/// Style of the navigation bar on android.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NavigationBar {
    /// The background color.
    ///
    /// # Platform
    /// - Only supported on `android`.
    pub color: Option<Color>,

    /// Whether the theme of the bar should be light.
    pub light: bool,
}
