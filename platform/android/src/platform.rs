use std::{
    pin::Pin,
    sync::{Arc, mpsc::Sender},
};

use jni::{Env, JValue, jni_sig, jni_str, objects::JObject, refs::Global, vm::JavaVM};
use ori::{Message, Proxied, Proxy};
use ori_native_core::Unsupported;

use crate::{
    application::{ACTIVITY, Event},
    widgets,
};

pub struct Platform {
    sender:   Sender<Event>,
    jvm:      JavaVM,
    activity: Arc<Global<JObject<'static>>>,

    next_id: u64,
}

impl Default for Platform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform {
    pub fn new() -> Self {
        let activity = ACTIVITY.get().unwrap();

        Self {
            sender:   activity.sender.clone(),
            jvm:      activity.jvm.clone(),
            activity: activity.activity.clone(),

            next_id: 0,
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

    pub fn remove_widget(&self, widget: WidgetId) {
        self.jni(|env, activity| {
            env.call_method(
                activity,
                jni_str!("removeView"),
                jni_sig!((long)),
                &[widget.into()],
            )?
            .z()
        })
        .unwrap();
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
    type Image = Unsupported;
    type Pressable = Unsupported;
    type Scroll = Unsupported;
    type Text = widgets::Text;
    type TextInput = Unsupported;
    type Transform = Unsupported;
    type Window = widgets::Window;

    fn quit(&mut self) {}
}

impl Proxied for Platform {
    type Proxy = AndroidProxy;

    fn proxy(&mut self) -> Self::Proxy {
        AndroidProxy {
            sender: self.sender.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AndroidProxy {
    sender: Sender<Event>,
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

    fn spawn_boxed(&self, _future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        todo!()
    }
}
