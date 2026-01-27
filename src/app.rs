use ori::Effect;

use crate::{Context, platform};

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
    pub fn run<T, V>(self, data: &mut T, mut ui: impl FnMut(&T) -> V)
    where
        V: Effect<Context, T> + 'static,
        V::State: 'static,
    {
        if tokio::runtime::Handle::try_current().is_ok() {
            panic!("`App::run` cannot be called from within an async runtime.");
        }

        self.native.run(data, move |data| Box::new(ui(data)));
    }
}
