use std::{pin::Pin, sync::Arc};

use ori::{Message, Proxied, Proxy};
use ori_native_core::Platform;
use tokio::sync::mpsc::UnboundedSender;

use crate::application::Event;

pub struct Gtk4Platform {
    pub(crate) proxy:       Gtk4Proxy,
    pub(crate) application: gtk4::Application,
}

impl Gtk4Platform {
    pub(crate) fn new(sender: UnboundedSender<Event>, application: gtk4::Application) -> Self {
        let runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());

        Self {
            proxy: Gtk4Proxy { sender, runtime },
            application,
        }
    }
}

impl Platform for Gtk4Platform {
    type Widget = gtk4::Widget;

    fn quit(&mut self) {
        let _ = self.proxy.sender.send(Event::Quit);
    }
}

impl Proxied for Gtk4Platform {
    type Proxy = Gtk4Proxy;

    fn proxy(&mut self) -> Self::Proxy {
        self.proxy.clone()
    }
}

#[derive(Clone)]
pub struct Gtk4Proxy {
    sender:  UnboundedSender<Event>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl Proxy for Gtk4Proxy {
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
