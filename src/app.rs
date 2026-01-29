use crate::{Effect, platform};

pub struct App {
    native: platform::Application,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            native: platform::Application::new(),
        }
    }

    #[track_caller]
    pub fn run<T, V>(self, data: &mut T, ui: impl FnMut(&T) -> V)
    where
        V: Effect<T>,
    {
        if tokio::runtime::Handle::try_current().is_ok() {
            panic!("`App::run` cannot be called from within an async runtime.");
        }

        self.native.run(data, ui);
    }
}
