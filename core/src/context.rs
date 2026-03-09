use std::{
    any::{self, Any, TypeId},
    convert::Infallible,
};

use ori::{Action, AnyView, Base, Message, Provider, Proxied, Proxy, Tracker, Tree, ViewId};

use crate::{AnimateRequest, BoxedWidget, LayoutRequest, Platform};

/// A leaf in the layout tree.
pub trait LayoutLeaf<P>: 'static {
    /// Compute the size for the given constraints.
    fn measure(
        &mut self,
        platform: &mut P,
        known_size: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32>;
}

impl<P> LayoutLeaf<P> for Infallible {
    fn measure(
        &mut self,
        _platform: &mut P,
        _known_size: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        unreachable!()
    }
}

/// The context of the [`View`](ori::View) tree.
pub struct Context<P> {
    /// The [`Platform`] of this context.
    pub platform:         P,
    layout_tree:          taffy::TaffyTree<Box<dyn LayoutLeaf<P>>>,
    layout_controller:    Option<ViewId>,
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
            layout_tree: taffy::TaffyTree::new(),
            layout_controller: None,
            animation_controller: None,
            resources: Vec::new(),
            view_id_tree: Tree::new(),
        }
    }

    /// Create a new layout node.
    pub fn new_layout_node(
        &mut self,
        style: taffy::Style,
        children: &[taffy::NodeId],
    ) -> taffy::NodeId {
        self.request_relayout();
        self.layout_tree
            .new_with_children(style, children)
            .expect("should never fail")
    }

    /// Create a new layout leaf.
    pub fn new_layout_leaf<T>(&mut self, style: taffy::Style, leaf: T) -> taffy::NodeId
    where
        T: LayoutLeaf<P> + 'static,
    {
        self.request_relayout();
        self.layout_tree
            .new_leaf_with_context(style, Box::new(leaf))
            .expect("should never fail")
    }

    /// Insert a child at `index` in a layout node.
    pub fn insert_layout_child(
        &mut self,
        parent: taffy::NodeId,
        index: usize,
        child: taffy::NodeId,
    ) -> taffy::TaffyResult<()> {
        self.request_relayout();
        self.layout_tree.insert_child_at_index(parent, index, child)
    }

    /// Replace the child at `index` in a layout node.
    pub fn replace_layout_child(
        &mut self,
        parent: taffy::NodeId,
        index: usize,
        child: taffy::NodeId,
    ) -> taffy::TaffyResult<()> {
        self.request_relayout();
        self.layout_tree
            .replace_child_at_index(parent, index, child)
            .map(|_| ())
    }

    /// Remove a layout node.
    pub fn remove_layout_node(&mut self, node: taffy::NodeId) -> taffy::TaffyResult<()> {
        self.request_relayout();
        self.layout_tree.remove(node).map(|_| ())
    }

    /// Remove the child at `index` from a layout node.
    pub fn remove_layout_child(
        &mut self,
        node: taffy::NodeId,
        index: usize,
    ) -> taffy::TaffyResult<()> {
        self.request_relayout();
        self.layout_tree
            .remove_child_at_index(node, index)
            .map(|_| {})
    }

    /// Set the layout style of a layout node.
    pub fn set_layout_style(
        &mut self,
        node: taffy::NodeId,
        style: taffy::Style,
    ) -> taffy::TaffyResult<()> {
        if let Ok(current) = self.layout_tree.style(node)
            && *current != style
        {
            self.request_relayout();
        }

        self.layout_tree.set_style(node, style)
    }

    /// Set the leaf of a layout.
    pub fn set_layout_leaf<T>(&mut self, node: taffy::NodeId, leaf: T) -> taffy::TaffyResult<()>
    where
        T: LayoutLeaf<P> + 'static,
    {
        self.request_relayout();
        self.layout_tree
            .set_node_context(node, Some(Box::new(leaf)))
    }

    /// Get the computed layout of a layout node.
    pub fn get_computed_layout(&self, node: taffy::NodeId) -> taffy::TaffyResult<&taffy::Layout> {
        self.layout_tree.layout(node)
    }

    /// Compute the layout of a layout tree with `node` as its root.
    pub fn compute_layout(
        &mut self,
        node: taffy::NodeId,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::TaffyResult<()>
    where
        P: Platform,
    {
        self.layout_tree.compute_layout_with_measure(
            node,
            available_space,
            |known_size, available_space, _node, context, _style| match context {
                Some(leaf) => leaf.measure(
                    &mut self.platform,
                    known_size,
                    available_space,
                ),

                None => taffy::Size::ZERO,
            },
        )
    }

    /// Request a relayout of the current layout controller.
    pub fn request_relayout(&mut self) {
        if let Some(layout_controller) = self.layout_controller.take() {
            self.platform.proxy().message(Message::new(
                LayoutRequest::Relayout,
                layout_controller,
            ));
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
        let previous = self.layout_controller.replace(view_id);
        let output = f(self);
        self.layout_controller = previous;
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
            if resource.type_id == TypeId::of::<T>() {
                continue;
            }

            let resource = self.resources.remove(i);
            return resource.value.downcast().ok();
        }

        None
    }

    fn get<T: Any>(&self) -> Option<&T> {
        for resource in self.resources.iter().rev() {
            if resource.type_id == TypeId::of::<T>() {
                continue;
            }

            return resource.value.downcast_ref();
        }

        None
    }

    fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        for resource in self.resources.iter_mut().rev() {
            if resource.type_id == TypeId::of::<T>() {
                continue;
            }

            return resource.value.downcast_mut();
        }

        None
    }
}
