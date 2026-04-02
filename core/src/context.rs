use std::{
    any::{self, Any, TypeId},
    collections::HashMap,
};

use ori::{Action, AnyView, Base, Message, Provider, Proxied, Proxy, Tracker, Tree, ViewId};

use crate::{
    AnimateRequest, BoxedWidget, LayoutRequest, LayoutTree, ModalRequest, Platform,
    native::NativeModal,
};

/// The context of the [`View`](ori::View) tree.
pub struct Context<P>
where
    P: Platform,
{
    /// The [`Platform`].
    pub platform: P,

    /// The [`LayoutTree`].
    pub layout: LayoutTree<P>,

    /// The modals in this context.
    pub modals: HashMap<ViewId, P::Modal>,

    animation_controller: Option<ViewId>,
    modal_controller:     Option<ViewId>,

    resources: Vec<Resource>,

    view_id_tree: Tree,
}

#[allow(dead_code)]
struct Resource {
    type_id:   TypeId,
    type_name: &'static str,
    value:     Box<dyn Any>,
}

impl<P> Context<P>
where
    P: Platform,
{
    /// Create a [`Context`] for a given [`Platform`].
    pub fn new(platform: P) -> Self {
        Self {
            platform,
            layout: LayoutTree::new(),
            animation_controller: None,
            modal_controller: None,
            modals: HashMap::new(),
            resources: Vec::new(),
            view_id_tree: Tree::new(),
        }
    }

    /// Request starting to animate.
    pub fn request_start_animating(&mut self) {
        if let Some(animation_controller) = self.animation_controller {
            self.platform.proxy().message(Message::new(
                AnimateRequest::Start,
                animation_controller,
            ));
        }
    }

    /// Request stopping animating.
    pub fn request_stop_animating(&mut self) {
        if let Some(animation_controller) = self.animation_controller {
            self.platform.proxy().message(Message::new(
                AnimateRequest::Stop,
                animation_controller,
            ));
        }
    }

    /// Open a modal, this will fail if no modal controller is set.
    pub fn open_modal(&mut self, id: ViewId, modal: P::Modal) -> bool {
        let Some(controller) = self.modal_controller else {
            modal.teardown(&mut self.platform);
            return false;
        };

        let request = ModalRequest::Open { id };
        let message = Message::new(request, Some(controller));
        self.proxy().message(message);
        self.modals.insert(id, modal);
        true
    }

    /// Close a modal, this will fail if no modal controller is set.
    pub fn close_modal(&mut self, id: ViewId) {
        let Some(controller) = self.modal_controller else {
            return;
        };

        let request = ModalRequest::Close { id };
        let message = Message::new(request, Some(controller));
        self.proxy().message(message);
    }

    /// Get a mutable reference to `self` and a modal at the same time.
    ///
    /// Returns `None` if the modal doesn't exist.
    pub fn with_modal<T>(
        &mut self,
        id: ViewId,
        f: impl FnOnce(&mut Self, &mut P::Modal) -> T,
    ) -> Option<T> {
        match self.modals.remove(&id) {
            Some(mut modal) => {
                let result = f(self, &mut modal);
                self.modals.insert(id, modal);
                Some(result)
            }

            None => None,
        }
    }

    /// Temporarily set the layout controller.
    ///
    /// This view will receive [`LayoutRequest`]s from its contents.
    pub fn with_layout_controller<T>(
        &mut self,
        view_id: ViewId,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let previous = self.layout.set_request_layout(Some(Box::new({
            let proxy = self.platform.proxy();
            move || {
                proxy.message(Message::new(
                    LayoutRequest::Layout,
                    view_id,
                ));
            }
        })));

        let output = f(self);

        self.layout.set_request_layout(previous);

        output
    }

    /// Temporarily set the modal controller.
    ///
    /// This view will receive [`ModalRequest`]s from its contents.
    pub fn with_modal_controller<T>(
        &mut self,
        view_id: ViewId,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let previous = self.modal_controller.replace(view_id);
        let output = f(self);
        self.modal_controller = previous;
        output
    }

    /// Temporarily set the animation controller.
    ///
    /// This view will receive [`AnimateRequest`]s from its contents.
    pub fn with_animation_controller<T>(
        &mut self,
        view_id: ViewId,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let previous = self.animation_controller.replace(view_id);
        let output = f(self);
        self.animation_controller = previous;
        output
    }

    /// Temporarily set the current window.
    ///
    /// This is a shorthand for setting both the layout and animation controller.
    pub fn with_window<T>(&mut self, view_id: ViewId, f: impl FnOnce(&mut Self) -> T) -> T {
        self.with_layout_controller(view_id, |this| {
            this.with_modal_controller(view_id, |this| {
                this.with_animation_controller(view_id, f)
            })
        })
    }
}

/// Type erased [`Effect`](ori::Effect).
pub type BoxedEffect<P, T> = Box<dyn AnyView<Context<P>, T, ()>>;

impl<P> Base for Context<P>
where
    P: Platform,
{
    type Element = BoxedWidget<P>;
}

impl<P> Tracker for Context<P>
where
    P: Platform,
{
    fn tree(&mut self) -> &mut Tree {
        &mut self.view_id_tree
    }
}

impl<P> Proxied for Context<P>
where
    P: Platform,
{
    type Proxy = P::Proxy;

    fn proxy(&mut self) -> Self::Proxy {
        self.platform.proxy()
    }

    fn send_action(&mut self, action: Action) {
        self.platform.send_action(action);
    }
}

impl Resource {
    fn is<T: Any>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    unsafe fn downcast_unchecked<T: Any>(self) -> Box<T> {
        let ptr: *mut T = Box::into_raw(self.value).cast();
        unsafe { Box::from_raw(ptr) }
    }

    unsafe fn downcast_ref_unchecked<T: Any>(&self) -> &T {
        let ptr = self.value.as_ref() as *const _ as *const T;
        unsafe { &*ptr }
    }

    unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T {
        let ptr = self.value.as_mut() as *mut _ as *mut T;
        unsafe { &mut *ptr }
    }
}

impl<P> Provider for Context<P>
where
    P: Platform,
{
    fn push<T: Any>(&mut self, resource: Box<T>) {
        self.resources.push(Resource {
            type_id:   TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
            value:     resource,
        });
    }

    fn pop<T: Any>(&mut self) -> Option<Box<T>> {
        let i = self.resources.iter().rposition(|r| r.is::<T>())?;

        let resource = self.resources.remove(i);
        let resource = unsafe { resource.downcast_unchecked::<T>() };
        Some(resource)
    }

    fn get<T: Any>(&self) -> Option<&T> {
        let resource = self.resources.iter().rev().find(|r| r.is::<T>())?;
        let resource = unsafe { resource.downcast_ref_unchecked::<T>() };
        Some(resource)
    }

    fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let resource = self.resources.iter_mut().rev().find(|r| r.is::<T>())?;
        let resource = unsafe { resource.downcast_mut_unchecked::<T>() };
        Some(resource)
    }
}
