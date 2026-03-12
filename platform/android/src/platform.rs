use std::{
    collections::HashMap,
    io,
    pin::Pin,
    sync::{Arc, mpsc::Sender},
    time::Duration,
};

use jni::{Env, JValue, jni_sig, jni_str, objects::JObject, refs::Global, vm::JavaVM};
use ori::{Message, Proxied, Proxy};

use crate::{
    application::{ACTIVITY, Event, WidgetEvent},
    widgets,
};

pub type EventHandler = Box<dyn FnMut(&WidgetEvent)>;

pub struct Platform {
    sender:   Sender<Event>,
    jvm:      JavaVM,
    activity: Arc<Global<JObject<'static>>>,
    handlers: HashMap<WidgetId, Vec<EventHandler>>,
    runtime:  Arc<tokio::runtime::Runtime>,

    on_animation_frame: Box<dyn Fn(Duration)>,

    next_id: u64,
}

impl Platform {
    pub fn new() -> io::Result<Self> {
        let activity = ACTIVITY.get().unwrap();
        let runtime = Arc::new(tokio::runtime::Runtime::new()?);

        Ok(Self {
            sender: activity.sender.clone(),
            jvm: activity.jvm.clone(),
            activity: activity.activity.clone(),
            handlers: HashMap::new(),
            runtime,

            on_animation_frame: Box::new(|_| {}),

            next_id: 0,
        })
    }

    pub fn on_animation_frame(&self, duration: Duration) {
        (self.on_animation_frame)(duration);
    }

    pub fn set_on_animation_frame(&mut self, f: impl Fn(Duration) + 'static) {
        self.on_animation_frame = Box::new(f);
    }

    pub fn add_handler(&mut self, widget: WidgetId, handler: impl FnMut(&WidgetEvent) + 'static) {
        self.handlers
            .entry(widget)
            .or_default()
            .push(Box::new(handler));
    }

    pub fn handle_event(&mut self, widget: WidgetId, event: WidgetEvent) {
        if let Some(handlers) = self.handlers.get_mut(&widget) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    pub fn jni<T, E>(
        &self,
        f: impl FnOnce(&mut Env<'_>, &JObject<'_>) -> Result<T, E>,
    ) -> Result<T, E>
    where
        E: From<jni::errors::Error>,
    {
        self.jvm.attach_current_thread(move |env| {
            let activity = self.activity.as_obj();
            f(env, activity)
        })
    }

    pub fn remove_widget(&mut self, widget: WidgetId) {
        self.handlers.remove(&widget);

        let _ = self.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("removeView"),
                jni_sig!((long)),
                &[widget.into()],
            )?
            .v()
        });
    }

    pub fn run_ui_tasks(&mut self) {
        let _ = self.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("runUiTasks"),
                jni_sig!(()),
                &[],
            )?
            .v()
        });
    }

    pub fn next_id(&mut self) -> WidgetId {
        let index = self.next_id;
        self.next_id += 1;
        WidgetId { index }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId {
    index: u64,
}

impl WidgetId {
    pub fn new(index: u64) -> Self {
        Self { index }
    }
}

impl<'a> From<WidgetId> for JValue<'a> {
    fn from(id: WidgetId) -> Self {
        JValue::Long(id.index as i64)
    }
}

impl<'a> From<&WidgetId> for JValue<'a> {
    fn from(id: &WidgetId) -> Self {
        JValue::Long(id.index as i64)
    }
}

impl ori_native_core::Platform for Platform {
    type Widget = WidgetId;

    type Group = widgets::Group;
    type Image = widgets::Image;
    type Pressable = widgets::Pressable;
    type Scroll = widgets::Scroll;
    type Text = widgets::Text;
    type TextInput = widgets::TextInput;
    type Transform = widgets::Transform;
    type Window = widgets::Window;

    fn quit(&mut self) {}
}

impl Proxied for Platform {
    type Proxy = AndroidProxy;

    fn proxy(&mut self) -> Self::Proxy {
        AndroidProxy {
            sender:  self.sender.clone(),
            runtime: self.runtime.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AndroidProxy {
    sender:  Sender<Event>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl Proxy for AndroidProxy {
    fn cloned(&self) -> Arc<dyn Proxy> {
        Arc::new(self.clone())
    }

    fn rebuild(&self) {
        let _ = self.sender.send(Event::Rebuild);
    }

    fn message(&self, message: Message) {
        let _ = self.sender.send(Event::Message(message));
    }

    fn spawn_boxed(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.runtime.spawn(future);
    }
}
