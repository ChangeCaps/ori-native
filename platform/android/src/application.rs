use std::{
    error, fmt, io,
    sync::{
        Arc, Mutex, OnceLock,
        mpsc::{Receiver, Sender},
    },
    time::Duration,
};

use jni::{objects::JObject, refs::Global, vm::JavaVM};
use ori::{Effect, Message, Proxied, Tracker};
use ori_native_core::{Context, native::Press};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

use crate::{Platform, log::MakeAndroidWriter, platform::WidgetId};

#[derive(Debug)]
pub enum Error {
    Uninitialized,
    MultipleApplications,
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Uninitialized => f.write_str("the ori runtime has not been initialized"),

            Error::MultipleApplications => f.write_str(
                "running multiple applications at the same time is not supported on android",
            ),

            Error::Io(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

pub struct Application {}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run<T, V>(self, data: &mut T, mut build: impl FnMut(&T) -> V) -> Result<(), Error>
    where
        V: Effect<Context<Platform>, T>,
    {
        let activity = ACTIVITY.get().ok_or(Error::Uninitialized)?;

        let mut receiver = activity
            .receiver
            .try_lock()
            .map_err(|_| Error::MultipleApplications)?;

        let platform = Platform::new(activity).map_err(Error::Io)?;
        let mut context = Context::new(platform);

        let view = build(data);

        context.tree().reset();
        let (_, state) = view.build(&mut context, data);

        let mut state = State {
            data,
            build,
            state,
            context,
            running: true,
            receiver: &mut receiver,
        };

        while state.running
            && let Ok(event) = state.receiver.recv()
        {
            state.handle_event(event);

            // we want to handle every task in the queue before updating the ui.
            while let Ok(event) = state.receiver.try_recv() {
                state.handle_event(event);
            }

            // after all events have been handled, and we expect to be idle,
            // execute batched ui updates.
            state.context.platform.run_ui_tasks();
        }

        state.teardown();

        Ok(())
    }

    pub fn init_log() {
        let mut filter = EnvFilter::default();

        if cfg!(debug_assertions) {
            filter = filter.add_directive(tracing::Level::DEBUG.into());
        }

        let subscriber = tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().with_writer(MakeAndroidWriter));

        let _ = tracing::subscriber::set_global_default(subscriber);
    }
}

pub static ACTIVITY: OnceLock<Activity> = OnceLock::new();

pub struct Activity {
    pub sender:   Sender<Event>,
    pub receiver: Mutex<Receiver<Event>>,
    pub jvm:      JavaVM,
    pub activity: Arc<Global<JObject<'static>>>,
}

impl Activity {
    pub fn event(&self, widget: WidgetId, event: WidgetEvent) {
        let _ = self.sender.send(Event::Widget(widget, event));
    }
}

#[derive(Debug)]
pub enum Event {
    Rebuild,
    Message(Message),
    Frame(Duration),
    Widget(WidgetId, WidgetEvent),
}

#[derive(Debug)]
pub enum WidgetEvent {
    Press(Press),
    Change(String),
    Submit(String),
}

struct State<'a, T, V, B>
where
    V: Effect<Context<Platform>, T>,
{
    data:     &'a mut T,
    build:    B,
    state:    V::State,
    context:  Context<Platform>,
    running:  bool,
    receiver: &'a mut Receiver<Event>,
}

impl<T, V, B> State<'_, T, V, B>
where
    V: Effect<Context<Platform>, T>,
    B: FnMut(&T) -> V,
{
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Rebuild => {
                let view = (self.build)(self.data);

                self.context.tree().reset();
                view.rebuild(
                    (),
                    &mut self.state,
                    &mut self.context,
                    self.data,
                );
            }

            Event::Message(mut message) => {
                self.context.tree().reset();
                let action = V::message(
                    (),
                    &mut self.state,
                    &mut self.context,
                    self.data,
                    &mut message,
                );

                self.context.send_action(action);
            }

            Event::Frame(duration) => self.context.platform.on_animation_frame(duration),

            Event::Widget(id, event) => self.context.platform.handle_event(id, event),
        }
    }

    fn teardown(mut self) {
        V::teardown((), self.state, &mut self.context);
    }
}
