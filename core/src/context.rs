use std::any::{self, Any, TypeId};

use ori::{Action, AnyView, Base, Message, Provider, Proxied, Proxy, Tracker, Tree, ViewId};

use crate::{AnimateRequest, BoxedWidget, LayoutRequest, LayoutTree, Platform};

/// The context of the [`View`](ori::View) tree.
pub struct Context<P> {
    /// The [`Platform`].
    pub platform: P,

    /// The [`LayoutTree`].
    pub layout: LayoutTree<P>,

    animation_controller: Option<ViewId>,
    resources:            Vec<Resource>,
    view_id_tree:         Tree,
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
            this.with_animation_controller(view_id, f)
        })
    }
}

/// Type erased [`Effect`](ori::Effect).
pub type BoxedEffect<P, T> = Box<dyn AnyView<Context<P>, T, ()>>;

impl<P> Base for Context<P> {
    type Element = BoxedWidget<P>;
}

impl<P> Tracker for Context<P> {
    fn tree(&mut self) -> &mut Tree {
        &mut self.view_id_tree
    }
}

impl<P> Proxied for Context<P>
where
    P: Proxied,
{
    type Proxy = P::Proxy;

    fn proxy(&mut self) -> Self::Proxy {
        self.platform.proxy()
    }

    fn send_action(&mut self, action: Action) {
        self.platform.send_action(action);
    }
}

impl<P> Provider for Context<P> {
    fn push<T: Any>(&mut self, resource: Box<T>) {
        self.resources.push(Resource {
            type_id:   TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
            value:     resource,
        });
    }

    fn pop<T: Any>(&mut self) -> Option<Box<T>> {
        for (i, resource) in self.resources.iter().enumerate().rev() {
            if resource.type_id != TypeId::of::<T>() {
                continue;
            }

            let resource = self.resources.remove(i);
            return resource.value.downcast().ok();
        }

        None
    }

    fn get<T: Any>(&self) -> Option<&T> {
        for resource in self.resources.iter().rev() {
            if resource.type_id != TypeId::of::<T>() {
                continue;
            }

            return resource.value.downcast_ref();
        }

        None
    }

    fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        for resource in self.resources.iter_mut().rev() {
            if resource.type_id != TypeId::of::<T>() {
                continue;
            }

            return resource.value.downcast_mut();
        }

        None
    }
}
