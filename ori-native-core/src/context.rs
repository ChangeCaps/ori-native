use std::any::Any;

use ori::{Action, AnyView, Base, Message, Provider, Proxied, Proxy, ViewId};

use crate::{BoxedWidget, Platform, views::WindowMessage};

pub trait LayoutLeaf<P>: 'static {
    fn measure(
        &mut self,
        platform: &mut P,
        known_size: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32>;
}

pub struct Context<P> {
    pub platform:         P,
    layout_tree:          taffy::TaffyTree<Box<dyn LayoutLeaf<P>>>,
    layout_controller:    Option<ViewId>,
    animation_controller: Option<ViewId>,
    resources:            Vec<Box<dyn Any>>,
}

impl<P> Context<P>
where
    P: Platform,
{
    pub fn new(platform: P) -> Self {
        Self {
            platform,
            layout_tree: taffy::TaffyTree::new(),
            layout_controller: None,
            animation_controller: None,
            resources: Vec::new(),
        }
    }

    pub fn new_layout_node(
        &mut self,
        style: taffy::Style,
        children: &[taffy::NodeId],
    ) -> taffy::NodeId {
        self.relayout();
        self.layout_tree
            .new_with_children(style, children)
            .expect("should never fail")
    }

    pub fn new_layout_leaf<T>(&mut self, style: taffy::Style, leaf: T) -> taffy::NodeId
    where
        T: LayoutLeaf<P> + 'static,
    {
        self.relayout();
        self.layout_tree
            .new_leaf_with_context(style, Box::new(leaf))
            .expect("should never fail")
    }

    pub fn insert_layout_child(
        &mut self,
        parent: taffy::NodeId,
        index: usize,
        child: taffy::NodeId,
    ) -> taffy::TaffyResult<()> {
        self.relayout();
        self.layout_tree.insert_child_at_index(parent, index, child)
    }

    pub fn replace_layout_child(
        &mut self,
        parent: taffy::NodeId,
        index: usize,
        child: taffy::NodeId,
    ) -> taffy::TaffyResult<()> {
        self.relayout();
        self.layout_tree
            .replace_child_at_index(parent, index, child)
            .map(|_| ())
    }

    pub fn remove_layout_node(&mut self, node: taffy::NodeId) -> taffy::TaffyResult<()> {
        self.relayout();
        self.layout_tree.remove(node).map(|_| ())
    }

    pub fn remove_layout_child(
        &mut self,
        node: taffy::NodeId,
        index: usize,
    ) -> taffy::TaffyResult<()> {
        self.relayout();
        self.layout_tree
            .remove_child_at_index(node, index)
            .map(|_| {})
    }

    pub fn set_layout_style(
        &mut self,
        node: taffy::NodeId,
        style: taffy::Style,
    ) -> taffy::TaffyResult<()> {
        if let Ok(current) = self.layout_tree.style(node)
            && *current != style
        {
            self.relayout();
        }

        self.layout_tree.set_style(node, style)
    }

    pub fn set_leaf_layout<T>(&mut self, node: taffy::NodeId, leaf: T) -> taffy::TaffyResult<()>
    where
        T: LayoutLeaf<P> + 'static,
    {
        self.relayout();
        self.layout_tree
            .set_node_context(node, Some(Box::new(leaf)))
    }

    pub fn get_computed_layout(&self, node: taffy::NodeId) -> taffy::TaffyResult<&taffy::Layout> {
        self.layout_tree.layout(node)
    }

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

    pub fn relayout(&mut self) {
        if let Some(layout_controller) = self.layout_controller.take() {
            self.platform.proxy().message(Message::new(
                WindowMessage::Relayout,
                layout_controller,
            ));
        }
    }

    pub fn start_animating(&mut self) {
        if let Some(animation_controller) = self.animation_controller {
            self.platform.proxy().message(Message::new(
                WindowMessage::StartAnimating,
                animation_controller,
            ));
        }
    }

    pub fn stop_animating(&mut self) {
        if let Some(animation_controller) = self.animation_controller {
            self.platform.proxy().message(Message::new(
                WindowMessage::StopAnimating,
                animation_controller,
            ));
        }
    }

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

    pub fn with_window<T>(&mut self, view_id: ViewId, f: impl FnOnce(&mut Self) -> T) -> T {
        self.with_layout_controller(view_id, |this| {
            this.with_animation_controller(view_id, f)
        })
    }
}

pub type BoxedEffect<P, T> = Box<dyn AnyView<Context<P>, T, ()>>;

impl<P> Base for Context<P> {
    type Element = BoxedWidget<P>;
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
        self.resources.push(resource);
    }

    fn pop<T: Any>(&mut self) -> Option<Box<T>> {
        let (index, _) = self
            .resources
            .iter()
            .enumerate()
            .rev()
            .find(|(_, resource)| resource.as_ref().is::<T>())?;

        self.resources.remove(index).downcast().ok()
    }

    fn get<T: Any>(&self) -> Option<&T> {
        self.resources
            .iter()
            .rev()
            .find_map(|resource| resource.downcast_ref())
    }

    fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources
            .iter_mut()
            .rev()
            .find_map(|resource| resource.downcast_mut())
    }
}
